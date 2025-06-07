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
use tracing::trace;

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
    pub fn client_state(&self) -> DownstreamConnectionState {
        self.state
    }
    pub fn client_state_mut(&mut self) -> &mut DownstreamConnectionState {
        &mut self.state
    }
    pub fn inner_state(&self) -> ClientState {
        self.inner.client_state()
    }
    pub fn inner_state_mut(&mut self) -> &mut ClientState {
        self.inner.client_state_mut()
    }
    pub async fn handle_handshake(&mut self) -> Result<(), Error> {
        use packets::handshake::serverbound::Handshake;

        let handshake = self.read_specific_packet::<Handshake>().await?;

        match handshake.next_state {
            ClientState::Status => {
                *self.client_state_mut() = DownstreamConnectionState::StatusRequest;
                *self.inner_state_mut() = ClientState::Status;
            }
            ClientState::Login => {
                *self.client_state_mut() = DownstreamConnectionState::LoginStart;
                *self.inner_state_mut() = ClientState::Login;
            }
            _ => {
                self.disconnect(Some(
                    serde_json::json!({ "text": "Received invalid handshake." }),
                ))
                .await?;
            }
        }

        Ok(())
    }
    pub async fn handle_status_ping(&mut self, online_player_count: usize) -> Result<(), Error> {
        // The state just changed from Handshake to Status.
        use base64::Engine;
        use packets::status::{
            clientbound::{PingResponse, StatusResponse},
            serverbound::{PingRequest, StatusRequest},
        };

        // Read the status request packet.
        let _status_request = self.read_specific_packet::<StatusRequest>().await?;

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
        let payload = self.read_specific_packet::<PingRequest>().await?.payload;

        // Send the ping response packet.
        self.send_packet(PingResponse { payload }).await?;

        self.disconnect(None).await?;

        Ok(())
    }
    pub async fn handle_login(&mut self) -> Result<(), Error> {
        // The state just changed from Handshake to Login.
        use packets::login::{clientbound::LoginSuccess, serverbound::LoginStart};

        // Read login start packet.
        let login_start = self.read_specific_packet::<LoginStart>().await?;

        // Enable encryption and authenticate with Mojang.
        // self.enable_encryption().await?;

        // Enable compression.
        self.enable_compression().await?;

        // Send login success packet.
        self.send_packet(LoginSuccess {
            // Generate a random UUID if none was provided.
            uuid: login_start.uuid.unwrap_or(uuid::Uuid::new_v4()),
            username: login_start.name,
            properties: vec![],
        })
        .await?;

        Ok(())
    }
    pub async fn enable_encryption(&mut self) -> Result<(), Error> {
        use crate::protocol::encryption::*;
        use packets::login::{clientbound::EncryptionRequest, serverbound::EncryptionResponse};
        use rand::{rngs::StdRng, Rng, SeedableRng};

        assert!(matches!(self.inner_state(), ClientState::Login));

        // RSA keys were generated on startup.
        let config = Config::instance();
        let (public_key, private_key) = &config.rsa_key_pair;
        tracing::trace!(
            "{}",
            public_key
                .serialize()
                .iter()
                .map(|b| format!("{b:02X?}"))
                .collect::<Vec<String>>()
                .join("")
        );

        // Generate a verify token.
        let mut rng = StdRng::from_entropy();
        let verify_token: [u8; 16] = rng.gen();

        // Send the encryption request packet.
        self.send_packet(EncryptionRequest {
            server_id: "".into(),
            public_key: public_key.serialize(),
            verify_token: verify_token.to_vec(),
            // TODO: Implement Mojang authentication.
            use_mojang_authentication: false,
        })
        .await?;

        // Read the encryption response packet.
        let encryption_response = self.read_specific_packet::<EncryptionResponse>().await?;

        // Verify the response.
        let decrypted_verify_token = private_key
            .decrypt(Pkcs1v15Encrypt, &encryption_response.verify_token)
            .expect("failed to decrypt verify token");
        if decrypted_verify_token != verify_token {
            return Err(Error::Invalid);
        }

        // Decrypt the shared secret.
        let shared_secret = private_key
            .decrypt(Pkcs1v15Encrypt, &encryption_response.shared_secret)
            .expect("failed to decrypt shared secret");

        // Enable encryption on the connection.
        trace!("Enabling encryption for connection {}", self.inner.id);
        todo!("Fix AES encryption implementation");
        let encryptor =
            Aes128Cfb8Encryptor::new((&(*shared_secret)).into(), (&(*shared_secret)).into());
        let decryptor =
            Aes128Cfb8Decryptor::new((&(*shared_secret)).into(), (&(*shared_secret)).into());
        self.inner.stream.codec_mut().aes_cipher = Some((encryptor, decryptor, 0));

        Ok(())
    }
    pub async fn enable_compression(&mut self) -> Result<(), Error> {
        // TODO: Implement compression.
        Ok(())
    }
    pub async fn read_packet(&mut self) -> Option<Result<Packet, Error>> {
        self.inner.read_packet().await
    }
    pub async fn send_packet<P: Into<Packet>>(&mut self, packet: P) -> Result<(), Error> {
        self.inner.send_packet(packet).await
    }
    pub async fn disconnect(&mut self, reason: Option<Chat>) -> Result<(), Error> {
        use packets::{
            configuration::clientbound::ConfigurationDisconnect,
            login::clientbound::LoginDisconnect, play::clientbound::PlayDisconnect,
        };

        // let reason = reason.unwrap_or(serde_json::json!({
        //     "text": "You have been disconnected!"
        // }));

        if let Some(reason) = reason {
            match self.inner_state() {
                ClientState::Disconnected | ClientState::Handshake | ClientState::Status => {
                    // Impossible to send a disconnect in these states.
                }
                ClientState::Login => {
                    let _ = self.send_packet(LoginDisconnect { reason }).await;
                }
                ClientState::Configuration => {
                    let _ = self.send_packet(ConfigurationDisconnect { reason }).await;
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
