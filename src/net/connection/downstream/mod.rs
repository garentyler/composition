pub mod manager;

use crate::{
    config::Config,
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
    pub async fn handle_status_ping(&mut self, online_player_count: usize) -> Result<(), Error> {
        // The state just changed from Handshake to Status.
        use base64::Engine;
        use packets::status::clientbound::{PingResponse, StatusResponse};

        // Read the status request packet.
        let Packet::StatusRequest(_status_request) =
            self.read_packet().await.ok_or(Error::Unexpected)??
        else {
            return Err(Error::Unexpected);
        };

        // Send the status response packet.
        let config = Config::instance();
        self.send_packet(StatusResponse {
            response: serde_json::json!({
                "version": {
                    "name": config.global.game_version,
                    "protocol": config.global.protocol_version
                },
                "players": {
                    "max": config.server.max_players,
                    "online": online_player_count,
                    "sample": []
                },
                "description": {
                    "text": config.server.motd
                },
                "favicon": format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD_NO_PAD.encode(&config.server.server_icon_bytes)),
                "enforcesSecureChat": false
            }),
        }).await?;

        // Read the ping request packet.
        let Packet::PingRequest(ping_request) =
            self.read_packet().await.ok_or(Error::Unexpected)??
        else {
            return Err(Error::Unexpected);
        };

        // Send the ping response packet.
        self.send_packet(PingResponse {
            payload: ping_request.payload,
        })
        .await?;

        self.disconnect(None).await
    }
    pub async fn read_packet(&mut self) -> Option<Result<Packet, Error>> {
        self.inner.read_packet().await
    }
    pub async fn send_packet<P: Into<Packet>>(&mut self, packet: P) -> Result<(), Error> {
        self.inner.send_packet(packet).await
    }
    pub async fn disconnect(&mut self, reason: Option<Chat>) -> Result<(), Error> {
        use packets::{login::clientbound::LoginDisconnect, play::clientbound::PlayDisconnect};

        // let reason = reason.unwrap_or(serde_json::json!({
        //     "text": "You have been disconnected!"
        // }));

        if let Some(reason) = reason {
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
