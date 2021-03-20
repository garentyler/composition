/// Definitions for all the packets in the Minecraft protocol.
pub mod packets;

use super::messages::*;
use crate::{mctypes::*, CONFIG, FAVICON};
use log::*;
use packets::*;
use serde_json::json;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;

/// The network client can only be in a few states,
/// this enum keeps track of that.
#[derive(PartialEq, Debug)]
pub enum NetworkClientState {
    Handshake,
    Status,
    Login,
    Play,
    Disconnected,
}

/// A wrapper to contain everything related
/// to networking for the client.
#[derive(Debug)]
pub struct NetworkClient {
    pub id: u128,
    pub connected: bool,
    pub stream: TcpStream,
    pub state: NetworkClientState,
    pub uuid: Option<String>,
    pub username: Option<String>,
    pub last_keep_alive: Instant,
    pub message_sender: Sender<ServerboundMessage>,
}
impl NetworkClient {
    /// Create a new `NetworkClient`
    pub fn new(
        stream: TcpStream,
        id: u128,
        message_sender: Sender<ServerboundMessage>,
    ) -> NetworkClient {
        NetworkClient {
            id,
            connected: true,
            stream,
            state: NetworkClientState::Handshake,
            uuid: None,
            username: None,
            last_keep_alive: Instant::now(),
            message_sender,
        }
    }

    /// Update the client.
    ///
    /// Updating could mean connecting new clients, reading packets,
    /// writing packets, or disconnecting clients.
    pub async fn update(&mut self, num_players: usize) -> tokio::io::Result<()> {
        // println!("{:?}", self);
        match self.state {
            NetworkClientState::Handshake => {
                let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).await?;
                let handshake = self.get_packet::<Handshake>().await?;
                // Minecraft versions 1.8 - 1.8.9 use protocol version 47.
                let compatible_versions = handshake.protocol_version == 47;
                let next_state = match handshake.next_state.into() {
                    1 => NetworkClientState::Status,
                    2 => NetworkClientState::Login,
                    _ => NetworkClientState::Disconnected,
                };
                self.state = next_state;
                // If incompatible versions or wrong next state
                if !compatible_versions {
                    let mut logindisconnect = LoginDisconnect::new();
                    logindisconnect.reason = MCChat {
                        text: MCString::from("Incompatible client! Server is on 1.8.9"),
                    };
                    self.send_packet(logindisconnect).await?;
                    self.state = NetworkClientState::Disconnected;
                }
            }
            NetworkClientState::Status => {
                let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).await?;
                let _statusrequest = self.get_packet::<StatusRequest>().await?;
                let mut statusresponse = StatusResponse::new();
                statusresponse.json_response = json!({
                    "version": {
                        "name": "1.8.9",
                        "protocol": 47,
                    },
                    "players": {
                        "max": CONFIG.max_players,
                        "online": num_players,
                        "sample": [
                            {
                                "name": "shvr",
                                "id": "e3f58380-60bb-4714-91f2-151d525e64aa"
                            }
                        ]
                    },
                    "description": {
                        "text": CONFIG.motd
                    },
                    "favicon": format!("data:image/png;base64,{}", if FAVICON.is_ok() { radix64::STD.encode(FAVICON.as_ref().unwrap().as_slice()) } else { "".to_owned() })
                })
                .to_string()
                .into();
                self.send_packet(statusresponse).await?;
                let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).await?;
                let statusping = self.get_packet::<StatusPing>().await?;
                let mut statuspong = StatusPong::new();
                statuspong.payload = statusping.payload;
                self.send_packet(statuspong).await?;
                self.state = NetworkClientState::Disconnected;
            }
            NetworkClientState::Login => {
                let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).await?;
                let loginstart = self.get_packet::<LoginStart>().await?;
                // Offline mode skips encryption and compression.
                // TODO: Encryption and compression
                let mut loginsuccess = LoginSuccess::new();
                // We're in offline mode, so this is a temporary uuid.
                // TODO: Get uuid and username from Mojang servers.
                loginsuccess.uuid = "00000000-0000-3000-0000-000000000000".into();
                loginsuccess.username = loginstart.player_name;
                self.uuid = Some(loginsuccess.uuid.clone().into());
                self.username = Some(loginsuccess.username.clone().into());
                self.send_packet(loginsuccess).await?;
                self.state = NetworkClientState::Play;
                let joingame = JoinGame::new();
                // TODO: Fill out `joingame` with actual information.
                self.send_packet(joingame).await?;
                let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).await?;
                let _clientsettings = self.get_packet::<ClientSettings>().await?;
                // TODO: Actually use client settings.
                let helditemchange = HeldItemChange::new();
                // TODO: Retrieve selected slot from storage.
                self.send_packet(helditemchange).await?;
                // TODO: S->C Declare Recipes (1.16?)
                // TODO: S->C Tags (1.16?)
                // TODO: S->C Entity Status (optional?)
                // TODO: S->C Declare Commands (1.16?)
                // TODO: S->C Unlock Recipes (1.16?)
                // TODO: S->C Player Position and Look
                let playerpositionandlook = ClientboundPlayerPositionAndLook::new();
                // TODO: Retrieve player position from storage.
                self.send_packet(playerpositionandlook).await?;
                // TODO: S->C Player Info (Add Player action) (1.16?)
                // TODO: S->C Player Info (Update latency action) (1.16?)
                // TODO: S->C Update View Position (1.16?)
                // TODO: S->C Update Light (1.16?)
                // TODO: S->C Chunk Data
                // TODO: S->C World Border
                // TODO: S->C Spawn Position
                let spawnposition = SpawnPosition::new();
                self.send_packet(spawnposition).await?;
                // Send initial keep alive.
                self.keep_alive().await?;
                // TODO: S->C Player Position and Look
                // TODO: C->S Teleport Confirm
                // TODO: C->S Player Position and Look
                // TODO: C->S Client Status
                // TODO: S->C inventories, entities, etc.
                self.message_sender
                    .send(ServerboundMessage::PlayerJoin(
                        self.uuid
                            .as_ref()
                            .unwrap_or(&"00000000-0000-3000-0000-000000000000".to_owned())
                            .to_string(),
                        self.username
                            .as_ref()
                            .unwrap_or(&"unknown".to_owned())
                            .to_string(),
                    ))
                    .expect("Message receiver disconnected");
            }
            NetworkClientState::Play => {
                if self.last_keep_alive.elapsed() > Duration::from_millis(1000) {
                    self.keep_alive().await?;
                }
                let (packet_length, packet_id) = read_packet_header(&mut self.stream).await?;
                // debug!("{}", packet_id);
                if packet_id == Player::id() {
                    let _player = self.get_packet::<Player>().await?;
                } else if packet_id == PlayerPosition::id() {
                    let _playerposition = self.get_packet::<PlayerPosition>().await?;
                } else if packet_id == PlayerLook::id() {
                    let _playerlook = self.get_packet::<PlayerLook>().await?;
                } else if packet_id == ServerboundPlayerPositionAndLook::id() {
                    let _playerpositionandlook = self
                        .get_packet::<ServerboundPlayerPositionAndLook>()
                        .await?;
                } else if packet_id == ServerboundChatMessage::id() {
                    let serverboundchatmessage =
                        self.get_packet::<ServerboundChatMessage>().await?;
                    let reply = format!("<{}> {}", self.get_name(), serverboundchatmessage.text);
                    info!("{}", reply);
                    self.message_sender
                        .send(ServerboundMessage::Chat(reply))
                        .expect("Message receiver disconnected");
                } else {
                    let _ = read_bytes(&mut self.stream, Into::<i32>::into(packet_length) as usize)
                        .await?;
                }
            }
            NetworkClientState::Disconnected => {
                if self.connected {
                    self.disconnect("Disconnected").await?;
                }
            }
        }
        Ok(())
    }

    /// Send a generic packet to the client.
    pub async fn send_packet<P: PacketCommon>(&mut self, packet: P) -> tokio::io::Result<()> {
        debug!("Sent {:?} {:#04X?} {:?}", self.state, P::id(), packet);
        packet.write(&mut self.stream).await
    }

    /// Read a generic packet from the network.
    pub async fn get_packet<T: PacketCommon>(&mut self) -> tokio::io::Result<T> {
        let packet = T::read(&mut self.stream).await?;
        debug!("Got {:?} {:#04X?} {:?}", self.state, T::id(), packet);
        Ok(packet)
    }

    /// Send the client a message in chat.
    pub async fn send_chat_message<C: Into<MCChat>>(
        &mut self,
        message: C,
    ) -> tokio::io::Result<()> {
        let mut chatmessage = ClientboundChatMessage::new();
        chatmessage.text = message.into();
        self.send_packet(chatmessage).await?;
        Ok(())
    }

    /// Disconnect the client.
    ///
    /// Sends `0x40 Disconnect` then waits 10 seconds before forcing the connection closed.
    pub async fn disconnect<S: Into<MCString>>(&mut self, reason: S) -> tokio::io::Result<()> {
        let mut disconnect = Disconnect::new();
        disconnect.reason.text = reason.into();
        self.send_packet(disconnect).await?;
        self.force_disconnect();
        Ok(())
    }

    /// Force disconnect the client by marking it for cleanup as disconnected.
    pub fn force_disconnect(&mut self) {
        self.connected = false;
        self.state = NetworkClientState::Disconnected;
    }

    /// Send a keep alive packet to the client.
    pub async fn keep_alive(&mut self) -> tokio::io::Result<()> {
        if cfg!(debug_assertions) {
            self.send_chat_message("keep alive").await?;
        }
        // Keep alive ping to client.
        let clientboundkeepalive = KeepAlivePing::new();
        self.send_packet(clientboundkeepalive).await?;
        // Keep alive pong to server.
        let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).await?;
        let _serverboundkeepalive = self.get_packet::<KeepAlivePong>().await?;
        self.last_keep_alive = Instant::now();
        Ok(())
    }

    /// Helper function to get the name of the player.
    pub fn get_name(&self) -> String {
        self.username
            .as_ref()
            .unwrap_or(&"unknown".to_owned())
            .to_string()
    }

    /// Receives broadcast messages from the server.
    pub async fn handle_broadcast_message(
        &mut self,
        message: BroadcastMessage,
    ) -> tokio::io::Result<()> {
        use BroadcastMessage::*;
        match message {
            Chat(s) => self.send_chat_message(s).await?,
            Disconnect(reason) => self.disconnect(reason).await?,
        }
        Ok(())
    }
}
