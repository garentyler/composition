/// Definitions for all the packets in the Minecraft protocol.
pub mod packets;

use crate::entity::player::Player;
use crate::{mctypes::*, CONFIG, FAVICON};
use log::{debug, info};
use packets::*;
use serde_json::json;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::{Duration, Instant};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

/// The struct containing all the data and running all the updates.
pub struct Server {
    network_clients: Vec<NetworkClient>,
    network_receiver: Receiver<NetworkClient>,
    pub players: Vec<Player>,
}
impl Server {
    pub fn new<A: 'static + ToSocketAddrs + Send>(addr: A) -> Server {
        let (tx, rx) = mpsc::channel();
        tokio::task::spawn(async move {
            let listener = TcpListener::bind(addr)
                .await
                .expect("Could not bind to TCP socket");
            let mut id = 0;
            loop {
                let (stream, _) = listener
                    .accept()
                    .await
                    .expect("Network receiver disconnected");
                tx.send(NetworkClient::new(stream, id as u128))
                    .expect("Network receiver disconnected");
                id += 1;
            }
        });
        info!("Network server started!");
        Server {
            network_receiver: rx,
            network_clients: vec![],
            players: vec![],
        }
    }

    /// Shut down the server.
    ///
    /// Disconnects all clients.
    pub async fn shutdown(&mut self) {
        info!("Server shutting down.");
        for client in self.network_clients.iter_mut() {
            let _ = client.disconnect(Some("The server is shutting down")).await;
            // We don't care if it doesn't succeed in sending the packet.
        }
    }

    /// Update the network server.
    ///
    /// Update each client in `self.network_clients`.
    async fn update_network(&mut self) -> tokio::io::Result<()> {
        loop {
            match self.network_receiver.try_recv() {
                Ok(client) => {
                    info!(
                        "Got client at {}",
                        client.stream.peer_addr().expect("Could not get peer addr")
                    );
                    self.network_clients.push(client)
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("Network sender disconnected"),
            }
        }
        let num_players = self.network_clients.iter().fold(0, |acc, nc| {
            if nc.state == NetworkClientState::Play {
                acc + 1
            } else {
                acc
            }
        });
        for client in self.network_clients.iter_mut() {
            if client.update(num_players).await.is_err() {
                client.force_disconnect();
            }
        }
        // Remove disconnected clients.
        self.network_clients
            .retain(|nc| nc.state != NetworkClientState::Disconnected);

        Ok(())
    }

    /// Update the game server.
    ///
    /// Start by updating the network.
    pub async fn update(&mut self) -> tokio::io::Result<()> {
        self.update_network().await?;
        Ok(())
    }
}

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
}
impl NetworkClient {
    /// Create a new `NetworkClient`
    pub fn new(stream: TcpStream, id: u128) -> NetworkClient {
        NetworkClient {
            id,
            connected: true,
            stream,
            state: NetworkClientState::Handshake,
            uuid: None,
            username: None,
            last_keep_alive: Instant::now(),
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
                debug!("{:?}", handshake);
            }
            NetworkClientState::Status => {
                let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).await?;
                let statusrequest = self.get_packet::<StatusRequest>().await?;
                debug!("{:?}", statusrequest);
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
                debug!("{:?}", statusping);
                let mut statuspong = StatusPong::new();
                statuspong.payload = statusping.payload;
                self.send_packet(statuspong).await?;
                self.state = NetworkClientState::Disconnected;
            }
            NetworkClientState::Login => {
                let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).await?;
                let loginstart = self.get_packet::<LoginStart>().await?;
                debug!("{:?}", loginstart);
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
                let clientsettings = self.get_packet::<ClientSettings>().await?;
                // TODO: Actually use client settings.
                debug!("{:?}", clientsettings);
                let helditemchange = HeldItemChange::new();
                // TODO: Retrieve selected slot from storage.
                self.send_packet(helditemchange).await?;
                // TODO: S->C Declare Recipes (1.16?)
                // TODO: S->C Tags (1.16?)
                // TODO: S->C Entity Status (optional?)
                // TODO: S->C Declare Commands (1.16?)
                // TODO: S->C Unlock Recipes (1.16?)
                // TODO: S->C Player Position and Look
                let playerpositionandlook = PlayerPositionAndLook::new();
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
                self.send_chat_message("keep alive").await?;
                self.keep_alive().await?;
                // TODO: S->C Player Position and Look
                // TODO: C->S Teleport Confirm
                // TODO: C->S Player Position and Look
                // TODO: C->S Client Status
                // TODO: S->C inventories, entities, etc.
                self.send_chat_message(format!(
                    "Welcome {} to the server!",
                    self.username.as_ref().unwrap_or(&"unknown".to_owned())
                ))
                .await?;
            }
            NetworkClientState::Play => {
                if self.last_keep_alive.elapsed() > Duration::from_millis(1000) {
                    self.send_chat_message("keep alive").await?;
                    self.keep_alive().await?;
                }
            }
            NetworkClientState::Disconnected => {
                if self.connected {
                    self.disconnect(None).await?;
                }
            }
        }
        Ok(())
    }

    /// Send a generic packet to the client.
    pub async fn send_packet<P: Into<Packet> + core::fmt::Debug>(
        &mut self,
        packet: P,
    ) -> tokio::io::Result<()> {
        debug!("{:?}", packet);
        Into::<Packet>::into(packet).write(&mut self.stream).await
    }

    /// Read a generic packet from the network.
    pub async fn get_packet<T: PacketCommon>(&mut self) -> tokio::io::Result<T> {
        Ok(T::read(&mut self.stream).await?)
    }

    /// Send the client a message in chat.
    async fn send_chat_message<C: Into<MCChat>>(&mut self, message: C) -> tokio::io::Result<()> {
        let mut chatmessage = ClientboundChatMessage::new();
        chatmessage.text = message.into();
        self.send_packet(chatmessage).await?;
        Ok(())
    }

    /// Disconnect the client.
    ///
    /// Sends `0x40 Disconnect` then waits 10 seconds before forcing the connection closed.
    async fn disconnect(&mut self, reason: Option<&str>) -> tokio::io::Result<()> {
        let mut disconnect = Disconnect::new();
        disconnect.reason.text = reason.unwrap_or("Disconnected").into();
        self.send_packet(disconnect).await?;
        self.force_disconnect();
        Ok(())
    }

    /// Force disconnect the client by marking it for cleanup as disconnected.
    fn force_disconnect(&mut self) {
        self.connected = false;
        self.state = NetworkClientState::Disconnected;
    }

    /// Send a keep alive packet to the client.
    async fn keep_alive(&mut self) -> tokio::io::Result<()> {
        // Keep alive ping to client.
        let clientboundkeepalive = KeepAlivePing::new();
        self.send_packet(clientboundkeepalive).await?;
        // Keep alive pong to server.
        let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).await?;
        let serverboundkeepalive = self.get_packet::<KeepAlivePong>().await?;
        debug!("{:?}", serverboundkeepalive);
        self.last_keep_alive = Instant::now();
        Ok(())
    }
}
