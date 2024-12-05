use crate::protocol::{
    packets::{self, Packet, PacketDirection},
    parsing::Parsable,
    ClientState,
};
use std::{collections::VecDeque, sync::Arc, time::Instant};
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpStream, sync::RwLock};
use tracing::{debug, trace, warn};

/// Similar to `composition_protocol::ClientState`,
/// but contains more useful data for managing the client's state.
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum NetworkClientState {
    /// A client has established a connection with the server.
    ///
    /// See `composition_protocol::ClientState::Handshake` for more details.
    Handshake,
    /// The client sent `SH00Handshake` with `next_state = ClientState::Status`
    /// and is performing [server list ping](https://wiki.vg/Server_List_Ping).
    Status {
        /// When the server receives `SS00StatusRequest`, this is set
        /// to `true` and the server should send `CS00StatusResponse`.
        received_request: bool,
        /// When the server receives `SS01PingRequest`, this is set
        /// to `true` and the server should send `CS01PingResponse`
        /// and set the connection state to `Disconnected`.
        received_ping: bool,
    },
    /// The client sent `SH00Handshake` with `next_state = ClientState::Login`
    /// and is attempting to join the server.
    Login {
        received_start: (bool, Option<packets::login::serverbound::LoginStart>),
    },
    /// The server sent `CL02LoginSuccess` and transitioned to `Play`.
    #[allow(dead_code)]
    Play,
    /// The client has disconnected.
    ///
    /// No packets should be sent or received,
    /// and the `NetworkClient` should be queued for removal.
    Disconnected,
}
impl From<NetworkClientState> for ClientState {
    fn from(value: NetworkClientState) -> Self {
        match value {
            NetworkClientState::Handshake => ClientState::Handshake,
            NetworkClientState::Status { .. } => ClientState::Status,
            NetworkClientState::Login { .. } => ClientState::Login,
            NetworkClientState::Play => ClientState::Play,
            NetworkClientState::Disconnected => ClientState::Disconnected,
        }
    }
}
impl AsRef<ClientState> for NetworkClientState {
    fn as_ref(&self) -> &ClientState {
        match self {
            NetworkClientState::Handshake => &ClientState::Handshake,
            NetworkClientState::Status { .. } => &ClientState::Status,
            NetworkClientState::Login { .. } => &ClientState::Login,
            NetworkClientState::Play => &ClientState::Play,
            NetworkClientState::Disconnected => &ClientState::Disconnected,
        }
    }
}

/// A wrapper around the raw `TcpStream` that abstracts away reading/writing packets and bytes.
#[derive(Debug, Clone)]
pub(crate) struct NetworkClient {
    /// The `NetworkClient`'s unique id.
    pub id: u128,
    pub state: NetworkClientState,
    stream: Arc<RwLock<TcpStream>>,
    /// Data gets appended to the back as it gets read,
    /// and popped from the front as it gets parsed into packets.
    incoming_data: VecDeque<u8>,
    /// Packets get appended to the back as they get read,
    /// and popped from the front as they get handled.
    pub incoming_packet_queue: VecDeque<Packet>,
    /// Keeps track of the last time the client sent data.
    ///
    /// This is useful for removing clients that have timed out.
    pub last_received_data_time: Instant,
    /// Packets get appended to the back and get popped from the front as they get sent.
    pub outgoing_packet_queue: VecDeque<Packet>,
}
impl NetworkClient {
    #[tracing::instrument]
    pub fn new(id: u128, stream: TcpStream) -> NetworkClient {
        NetworkClient {
            id,
            state: NetworkClientState::Handshake,
            stream: Arc::new(RwLock::new(stream)),
            incoming_data: VecDeque::new(),
            incoming_packet_queue: VecDeque::new(),
            last_received_data_time: Instant::now(),
            outgoing_packet_queue: VecDeque::new(),
        }
    }
    #[tracing::instrument]
    async fn read_data(&mut self) -> tokio::io::Result<()> {
        trace!("NetworkClient.read_data() id {}", self.id);
        let stream = self.stream.read().await;

        // Try to read 4kb at a time until there is no more data.
        loop {
            let mut buf = [0; 4096];

            let num_bytes = match stream.try_read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => return Err(e),
            };

            debug!("Read {} bytes from client {}", num_bytes, self.id);

            self.last_received_data_time = Instant::now();
            self.incoming_data.extend(&buf[..num_bytes]);
        }

        trace!("NetworkClient.read_data() end id {}", self.id);
        Ok(())
    }
    // TODO: Stream compression/encryption.
    #[tracing::instrument]
    pub async fn read_packets(&mut self) -> crate::protocol::Result<()> {
        trace!("NetworkClient.read_packet() id {}", self.id);

        if self.read_data().await.is_err() {
            self.disconnect(None).await;
            return Err(crate::protocol::Error::Disconnected);
        }

        self.incoming_data.make_contiguous();
        let (mut data, &[..]) = self.incoming_data.as_slices();

        let mut bytes_consumed = 0;
        while !data.is_empty() {
            let p = Packet::parse(
                self.state.clone().into(),
                PacketDirection::Serverbound,
                data,
            );
            trace!("{} got {:?}", self.id, p);
            match p {
                Ok((d, packet)) => {
                    debug!("Got packet {:?} from client {}", packet, self.id);
                    bytes_consumed += data.len() - d.len();
                    data = d;
                    self.incoming_packet_queue.push_back(packet);
                }
                Err(nom::Err::Incomplete(_)) => break,
                Err(_) => {
                    // Remove the valid bytes before this packet.
                    self.incoming_data = self.incoming_data.split_off(bytes_consumed);
                    return Err(crate::protocol::Error::Parsing);
                }
            }
        }

        // Remove the bytes we just read.
        self.incoming_data = self.incoming_data.split_off(bytes_consumed);

        Ok(())
    }
    // None: There was no packet to read.
    // Some(Err(())): The packet was the wrong type.
    // Some(Ok(_)): The packet was successfully read.
    #[tracing::instrument]
    pub fn read_packet<P: std::fmt::Debug + TryFrom<Packet>>(
        &mut self,
    ) -> Option<std::result::Result<P, Packet>> {
        if let Some(generic_packet) = self.incoming_packet_queue.pop_back() {
            if let Ok(packet) = TryInto::<P>::try_into(generic_packet.clone()) {
                Some(Ok(packet))
            } else {
                self.incoming_packet_queue.push_back(generic_packet.clone());
                Some(Err(generic_packet))
            }
        } else {
            None
        }
    }
    #[tracing::instrument]
    pub fn queue_packet<P: std::fmt::Debug + Into<Packet>>(&mut self, packet: P) {
        self.outgoing_packet_queue.push_back(packet.into());
    }
    #[tracing::instrument]
    pub async fn send_queued_packets(&mut self) -> crate::protocol::Result<()> {
        let packets: Vec<_> = self.outgoing_packet_queue.drain(..).collect();
        for packet in packets {
            self.send_packet(packet)
                .await
                .map_err(|_| crate::protocol::Error::Disconnected)?;
        }
        Ok(())
    }
    #[tracing::instrument]
    pub async fn send_packet<P: std::fmt::Debug + Into<Packet>>(
        &self,
        packet: P,
    ) -> tokio::io::Result<()> {
        let packet: Packet = packet.into();

        debug!("Sending packet {:?} to client {}", packet, self.id);
        let (packet_id, mut packet_body) = packet.serialize();
        let mut packet_id = packet_id.serialize();

        // TODO: Stream compression/encryption.

        let mut b = vec![];
        b.append(&mut packet_id);
        b.append(&mut packet_body);

        // bytes: packet length as varint, packet id as varint, packet body
        let bytes = Parsable::serialize(&b);

        self.stream.write().await.write_all(&bytes).await?;
        Ok(())
    }
    #[tracing::instrument]
    pub async fn disconnect(&mut self, reason: Option<crate::protocol::types::Chat>) {
        use packets::{login::clientbound::LoginDisconnect, play::clientbound::PlayDisconnect};

        let reason = reason.unwrap_or(serde_json::json!({
            "text": "You have been disconnected!"
        }));

        match self.state.as_ref() {
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

        self.state = NetworkClientState::Disconnected;
    }
}
