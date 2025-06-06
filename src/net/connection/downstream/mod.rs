pub mod manager;

use crate::{
    net::{connection::GenericConnection, error::Error},
    protocol::{
        packets::{self, Packet, PacketDirection},
        types::Chat,
        ClientState,
    },
};
use tokio::net::TcpStream;

/// The connection's current state.
/// Similar to crate::protocol::ClientState,
/// but has more fine-grained tracking for packet responses.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum DownstreamConnectionState {
    #[default]
    Handshake,
    StatusRequest,
    StatusPing,
    LoginStart,
    EncryptionResponse,
    LoginPluginResponse,
    Play,
    Disconnected,
}

#[derive(Debug)]
pub struct DownstreamConnection {
    inner: GenericConnection,
    state: DownstreamConnectionState,
}
impl DownstreamConnection {
    pub fn new(id: u128, stream: TcpStream) -> Self {
        DownstreamConnection {
            // receiving_direction: PacketDirection::Serverbound
            inner: GenericConnection::new(id, PacketDirection::Serverbound, stream),
            state: DownstreamConnectionState::Handshake,
        }
    }
    pub async fn read_packet(&mut self) -> Option<Result<Packet, Error>> {
        self.inner.read_packet().await
    }
    pub async fn send_packet<P: Into<Packet>>(&mut self, packet: P) -> Result<(), Error> {
        self.inner.send_packet(packet).await
    }
    pub async fn disconnect(mut self, reason: Option<Chat>) -> Result<(), Error> {
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

        self.inner.disconnect().await
    }
}
impl std::ops::Deref for DownstreamConnection {
    type Target = GenericConnection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DownstreamConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DownstreamConnection> for GenericConnection {
    fn from(value: DownstreamConnection) -> Self {
        value.inner
    }
}
