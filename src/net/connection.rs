use super::codec::PacketCodec;
use crate::protocol::{
    packets::{self, Packet, PacketDirection},
    types::Chat,
    ClientState,
};
use futures::{stream::StreamExt, SinkExt};
use std::time::{Duration, Instant};
use tokio::{io::BufStream, net::TcpStream};
use tokio_util::codec::{Decoder, Framed};
use tracing::trace;

#[derive(Debug)]
pub struct Connection {
    /// The `Connection`'s unique id.
    pub id: u128,
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
    pub fn client_state(&self) -> ClientState {
        self.stream.codec().client_state
    }
    pub fn received_elapsed(&self) -> Duration {
        self.last_received_data_time.elapsed()
    }
    pub fn sent_elapsed(&self) -> Duration {
        self.last_sent_data_time.elapsed()
    }
    pub async fn read_packet(&mut self) -> Option<Result<Packet, std::io::Error>> {
        self.last_received_data_time = Instant::now();
        self.stream.next().await
    }
    pub async fn send_packet<P: Into<Packet>>(&mut self, packet: P) -> Result<(), std::io::Error> {
        let packet: Packet = packet.into();
        self.stream.send(packet).await
    }
    pub async fn disconnect(mut self, reason: Option<Chat>) -> Result<(), std::io::Error> {
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
