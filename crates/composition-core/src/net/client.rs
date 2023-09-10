use std::{sync::Arc, time::Instant};

use crate::error::{Error, Result};
use composition_protocol::{
    packets::{codec::PacketCodec, serverbound::SL00LoginStart, Packet},
    prelude::*,
};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::codec::{Decoder, Framed};

/// Similar to `composition_protocol::ClientState`,
/// but contains more useful data for managing the client's state.
#[derive(Clone, PartialEq, Debug)]
pub enum NetworkClientState {
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
        received_start: (bool, Option<SL00LoginStart>),
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

#[derive(Debug)]
pub struct NetworkClient {
    pub id: u128,
    pub state: NetworkClientState,
    pub codec_state: Arc<Mutex<ClientState>>,
    pub packet_stream: Arc<Mutex<Framed<TcpStream, PacketCodec>>>,
    pub last_received_data_time: Mutex<Instant>,
}
impl NetworkClient {
    pub fn new(id: u128, stream: TcpStream) -> NetworkClient {
        let codec_state = Arc::new(Mutex::new(ClientState::Handshake));
        let codec = PacketCodec::new()
            .compression(false)
            .server()
            .client_state(codec_state.clone())
            .build();

        NetworkClient {
            id,
            state: NetworkClientState::Handshake,
            codec_state,
            packet_stream: Arc::new(Mutex::new(codec.framed(stream))),
            last_received_data_time: Mutex::new(Instant::now()),
        }
    }
    pub async fn read_packet(&self) -> Result<Packet> {
        let mut stream = self.packet_stream.lock().await;
        match stream.next().await {
            Some(Ok(packet)) => {
                *self.last_received_data_time.lock().await = Instant::now();
                Ok(packet)
            }
            Some(Err(composition_protocol::Error::Disconnected)) | None => {
                Err(Error::Protocol(composition_protocol::Error::Disconnected))
            }
            Some(Err(error)) => Err(Error::Protocol(error)),
        }
    }
    pub async fn read_typed_packet<P: TryFrom<Packet>>(
        &self,
    ) -> Result<std::result::Result<P, Packet>> {
        let packet = self.read_packet().await?;
        if let Ok(packet) = TryInto::<P>::try_into(packet.clone()) {
            Ok(Ok(packet))
        } else {
            Ok(Err(packet))
        }
    }
    pub async fn send_packet<P: Into<Packet>>(&self, packet: P) -> Result<()> {
        let packet: Packet = packet.into();
        self.packet_stream
            .lock()
            .await
            .send(packet)
            .await
            .map_err(Error::Protocol)
    }
    pub async fn disconnect(&mut self, reason: Option<Chat>) {
        use composition_protocol::packets::clientbound::{CL00Disconnect, CP17Disconnect};
        let reason = reason.unwrap_or(Chat::basic("You have been disconnected!"));

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
