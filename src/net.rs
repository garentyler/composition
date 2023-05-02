use crate::prelude::*;
use composition_protocol::{packet::GenericPacket, ClientState, ProtocolError};
use std::sync::Arc;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NetworkClientState {
    Handshake,
    Status {
        received_request: bool,
        received_ping: bool,
    },
    Login,
    Play,
    Disconnected,
}
impl From<NetworkClientState> for ClientState {
    fn from(value: NetworkClientState) -> Self {
        match value {
            NetworkClientState::Handshake => ClientState::Handshake,
            NetworkClientState::Status {
                received_request: _,
                received_ping: _,
            } => ClientState::Status,
            NetworkClientState::Login => ClientState::Login,
            NetworkClientState::Play => ClientState::Play,
            NetworkClientState::Disconnected => ClientState::Disconnected,
        }
    }
}
impl AsRef<ClientState> for NetworkClientState {
    fn as_ref(&self) -> &ClientState {
        match self {
            NetworkClientState::Handshake => &ClientState::Handshake,
            NetworkClientState::Status {
                received_request: _,
                received_ping: _,
            } => &ClientState::Status,
            NetworkClientState::Login => &ClientState::Login,
            NetworkClientState::Play => &ClientState::Play,
            NetworkClientState::Disconnected => &ClientState::Disconnected,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkClient {
    pub id: u128,
    pub state: NetworkClientState,
    stream: Arc<RwLock<TcpStream>>,
    incoming_data: VecDeque<u8>,
    pub incoming_packet_queue: VecDeque<GenericPacket>,
    pub last_received_data_time: Instant,
    pub outgoing_packet_queue: VecDeque<GenericPacket>,
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
    pub async fn read_packets(&mut self) -> composition_protocol::Result<()> {
        trace!("NetworkClient.read_packet() id {}", self.id);

        if self.read_data().await.is_err() {
            self.disconnect(None).await;
            return Err(ProtocolError::Disconnected);
        }

        self.incoming_data.make_contiguous();
        let (mut data, &[..]) = self.incoming_data.as_slices();

        let mut bytes_consumed = 0;
        while !data.is_empty() {
            let p = GenericPacket::parse_uncompressed(self.state.into(), true, data);
            trace!("{} got {:?}", self.id, p);
            match p {
                Ok((d, packet)) => {
                    debug!("Got packet {:?} from client {}", packet, self.id);
                    bytes_consumed += data.len() - d.len();
                    data = d;
                    self.incoming_packet_queue.push_back(packet);
                }
                Err(ProtocolError::NotEnoughData) => break,
                Err(e) => {
                    // Remove the valid bytes before this packet.
                    self.incoming_data = self.incoming_data.split_off(bytes_consumed);
                    return Err(e);
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
    pub fn read_packet<P: std::fmt::Debug + TryFrom<GenericPacket>>(
        &mut self,
    ) -> Option<Result<P, GenericPacket>> {
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
    pub fn queue_packet<P: std::fmt::Debug + Into<GenericPacket>>(&mut self, packet: P) {
        self.outgoing_packet_queue.push_back(packet.into());
    }
    #[tracing::instrument]
    pub async fn send_queued_packets(&mut self) -> composition_protocol::Result<()> {
        let packets: Vec<_> = self.outgoing_packet_queue.drain(..).collect();
        for packet in packets {
            self.send_packet(packet)
                .await
                .map_err(|_| ProtocolError::Disconnected)?;
        }
        Ok(())
    }
    #[tracing::instrument]
    pub async fn send_packet<P: std::fmt::Debug + Into<GenericPacket>>(
        &self,
        packet: P,
    ) -> tokio::io::Result<()> {
        use composition_protocol::util::serialize_varint;
        let packet: GenericPacket = packet.into();

        debug!("Sending packet {:?} to client {}", packet, self.id);
        let (packet_id, mut packet_body) = packet.serialize();
        let mut packet_id = serialize_varint(packet_id);

        // TODO: Stream compression/encryption.

        let mut b = vec![];
        b.append(&mut packet_id);
        b.append(&mut packet_body);

        // bytes: packet length as varint, packet id as varint, packet body
        let mut bytes = serialize_varint(b.len() as i32);
        bytes.append(&mut b);

        self.stream.write().await.write_all(&bytes).await?;
        Ok(())
    }
    #[tracing::instrument]
    pub async fn disconnect(&mut self, reason: Option<composition_protocol::Chat>) {
        use composition_protocol::packet::clientbound::{CL00Disconnect, CP17Disconnect};
        let reason = reason.unwrap_or(json!({
            "text": "You have been disconnected!"
        }));

        match self.state.as_ref() {
            ClientState::Disconnected | ClientState::Handshake | ClientState::Status => {
                // Impossible to send a disconnect in these states.
            }
            ClientState::Login => {
                let _ = self.send_packet(CL00Disconnect { reason }).await;
            }
            ClientState::Play => {
                let _ = self.send_packet(CP17Disconnect { reason }).await;
            }
        }

        self.state = NetworkClientState::Disconnected;
    }
}
