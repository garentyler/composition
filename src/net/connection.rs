use super::{codec::PacketCodec, error::Error};
use crate::protocol::{
    packets::{self, Packet, PacketDirection},
    types::Chat,
    ClientState,
};
use futures::{stream::StreamExt, SinkExt};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::{io::BufStream, net::TcpStream, sync::mpsc};
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    task::JoinHandle,
};
use tokio_util::codec::{Decoder, Framed};
use tokio_util::sync::CancellationToken;
use tracing::{error, trace};

#[derive(Debug)]
pub struct ConnectionManager {
    max_clients: Option<usize>,
    clients: HashMap<u128, Connection>,
    channel: (
        mpsc::UnboundedSender<Connection>,
        mpsc::UnboundedReceiver<Connection>,
    ),
}
impl ConnectionManager {
    pub fn new(max_clients: Option<usize>) -> ConnectionManager {
        ConnectionManager {
            max_clients,
            clients: HashMap::new(),
            channel: mpsc::unbounded_channel(),
        }
    }
    pub fn client(&self, id: u128) -> Option<&Connection> {
        self.clients.get(&id)
    }
    pub fn client_mut(&mut self, id: u128) -> Option<&mut Connection> {
        self.clients.get_mut(&id)
    }
    pub fn clients(&self) -> impl Iterator<Item = &Connection> {
        self.clients.values()
    }
    pub fn clients_mut(&mut self) -> impl Iterator<Item = &mut Connection> {
        self.clients.values_mut()
    }
    pub async fn spawn_listener<A>(
        &self,
        bind_address: A,
        running: CancellationToken,
    ) -> Result<JoinHandle<()>, Error>
    where
        A: 'static + ToSocketAddrs + Send + std::fmt::Debug,
    {
        trace!("Starting listener task");
        let fmt_addr = format!("{:?}", bind_address);
        let listener = TcpListener::bind(bind_address)
            .await
            .map_err(Error::Io)
            .inspect_err(|_| error!("Could not bind to {}.", fmt_addr))?;

        let sender = self.channel.0.clone();

        let join_handle = tokio::spawn(async move {
            let mut client_id = 0u128;

            loop {
                tokio::select! {
                    _ = running.cancelled() => {
                        break;
                    }
                    result = listener.accept() => {
                        if let Ok((stream, _)) = result {
                            trace!("Listener task got connection (id {})", client_id);
                            let client = Connection::new_client(client_id, stream);
                            if sender.send(client).is_err() {
                                trace!("Client receiver disconnected");
                                break;
                            }
                            client_id += 1;
                        }
                    }
                }
            }
            trace!("Listener task shutting down");
        });

        Ok(join_handle)
    }
    pub async fn update(&mut self) -> Result<(), Error> {
        // Receive new clients from the sender.
        loop {
            match self.channel.1.try_recv() {
                Ok(connection) => {
                    let id = connection.id();

                    match self.max_clients {
                        Some(max) => {
                            if self.clients.len() >= max {
                                let _ = connection.disconnect(None).await;
                            } else {
                                self.clients.insert(id, connection);
                            }
                        }
                        None => {
                            self.clients.insert(id, connection);
                        }
                    }
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    return Err(Error::ConnectionChannelDisconnnection)
                }
                Err(mpsc::error::TryRecvError::Empty) => break,
            };
        }

        // Disconnect any clients that have timed out.
        // We don't actually care if the disconnections succeed,
        // the connection is going to be dropped anyway.
        let _ = futures::future::join_all({
            // Workaround until issue #59618 hash_extract_if gets stabilized.
            let ids = self
                .clients
                .iter()
                .filter_map(|(id, c)| {
                    if c.received_elapsed() > Duration::from_secs(10) {
                        Some(*id)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            ids.into_iter()
                .map(|id| self.clients.remove(&id).unwrap())
                .map(|client| client.disconnect(None))
        })
        .await;

        // Remove disconnected clients.
        let before = self.clients.len();
        self.clients
            .retain(|_id, c| c.client_state() != ClientState::Disconnected);
        let after = self.clients.len();
        if before - after > 0 {
            trace!("Removed {} disconnected clients", before - after);
        }
        Ok(())
    }
    pub async fn disconnect(
        &mut self,
        id: u128,
        reason: Option<Chat>,
    ) -> Option<Result<(), Error>> {
        let client = self.clients.remove(&id)?;
        Some(client.disconnect(reason).await)
    }
    pub async fn shutdown(mut self, reason: Option<Chat>) -> Result<(), Error> {
        let reason = reason.unwrap_or(serde_json::json!({
            "text": "You have been disconnected!"
        }));

        let disconnections = self
            .clients
            .drain()
            .map(|(_, c)| c)
            .map(|c| c.disconnect(Some(reason.clone())))
            .collect::<Vec<_>>();

        // We don't actually care if the disconnections succeed,
        // the connection is going to be dropped anyway.
        let _disconnections = futures::future::join_all(disconnections).await;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Connection {
    /// The `Connection`'s unique id.
    id: u128,
    stream: Framed<BufStream<TcpStream>, PacketCodec>,
    last_received_data_time: Instant,
    last_sent_data_time: Instant,
}
impl Connection {
    fn new(id: u128, receiving_direction: PacketDirection, stream: TcpStream) -> Self {
        let codec = PacketCodec::new(ClientState::Handshake, receiving_direction);

        Connection {
            id,
            stream: codec.framed(BufStream::new(stream)),
            last_received_data_time: Instant::now(),
            last_sent_data_time: Instant::now(),
        }
    }
    pub fn new_client(id: u128, stream: TcpStream) -> Self {
        Self::new(id, PacketDirection::Serverbound, stream)
    }
    pub fn new_server(id: u128, stream: TcpStream) -> Self {
        Self::new(id, PacketDirection::Clientbound, stream)
    }
    pub fn id(&self) -> u128 {
        self.id
    }
    pub fn client_state(&self) -> ClientState {
        self.stream.codec().client_state
    }
    pub fn client_state_mut(&mut self) -> &mut ClientState {
        &mut self.stream.codec_mut().client_state
    }
    pub fn received_elapsed(&self) -> Duration {
        self.last_received_data_time.elapsed()
    }
    pub fn sent_elapsed(&self) -> Duration {
        self.last_sent_data_time.elapsed()
    }
    pub async fn read_packet(&mut self) -> Option<Result<Packet, Error>> {
        self.last_received_data_time = Instant::now();
        self.stream.next().await.map(|packet| {
            packet.map_err(|mut e| {
                // Set the codec error to something more descriptive.
                if e.to_string() == "bytes remaining on stream" {
                    e = Error::Io(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, e));
                }
                trace!("Error reading packet from connection {}: {:?}", self.id, e);
                e
            })
        })
    }
    pub async fn send_packet<P: Into<Packet>>(&mut self, packet: P) -> Result<(), Error> {
        let packet: Packet = packet.into();
        self.stream.send(packet).await.inspect_err(|e| {
            trace!("Error sending packet to connection {}: {:?}", self.id, e);
        })
    }
    pub async fn disconnect(mut self, reason: Option<Chat>) -> Result<(), Error> {
        trace!("Connection disconnected (id {})", self.id);
        use packets::{login::clientbound::LoginDisconnect, play::clientbound::PlayDisconnect};

        let reason = reason.unwrap_or(serde_json::json!({
            "text": "You have been disconnected!"
        }));

        match self.client_state() {
            ClientState::Disconnected | ClientState::Handshake | ClientState::Status => {
                // Impossible to send a disconnect in these states.
            }
            ClientState::Login => {
                let _ = self.send_packet(LoginDisconnect { reason }).await;
            }
            ClientState::Play => {
                let _ = self.send_packet(PlayDisconnect { reason }).await;
            }
        }

        self.stream.flush().await?;
        self.stream.codec_mut().client_state = ClientState::Disconnected;
        Ok(())
    }
}
