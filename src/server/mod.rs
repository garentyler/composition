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

    /// Update the network server.
    ///
    /// Update each client in `self.network_clients`.
    async fn update_network(&mut self) {
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
            client.update(num_players).await;
        }
        // Remove disconnected clients.
        self.network_clients
            .retain(|nc| nc.state != NetworkClientState::Disconnected);
    }

    /// Update the game server.
    ///
    /// Start by updating the network.
    pub async fn update(&mut self) {
        self.update_network().await;
    }
}

/// The network client can only be in a few states,
/// this enum keeps track of that.
#[derive(PartialEq)]
pub enum NetworkClientState {
    Handshake,
    Status,
    Login,
    Play,
    Disconnected,
}

/// A wrapper to contain everything related
/// to networking for the client.
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
    pub async fn update(&mut self, num_players: usize) {
        match self.state {
            NetworkClientState::Handshake => {
                let (_packet_length, _packet_id) =
                    read_packet_header(&mut self.stream).await.unwrap();
                let handshake = Handshake::read(&mut self.stream).await.unwrap();
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
                    logindisconnect.write(&mut self.stream).await.unwrap();
                    self.state = NetworkClientState::Disconnected;
                }
                debug!("Got handshake: {:?}", handshake);
            }
            NetworkClientState::Status => {
                let (_packet_length, _packet_id) =
                    read_packet_header(&mut self.stream).await.unwrap();
                let statusrequest = StatusRequest::read(&mut self.stream).await.unwrap();
                debug!("Got status request: {:?}", statusrequest);
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
                statusresponse.write(&mut self.stream).await.unwrap();
                debug!("Sending status response: StatusResponse");
                let (_packet_length, _packet_id) =
                    read_packet_header(&mut self.stream).await.unwrap();
                let statusping = StatusPing::read(&mut self.stream).await.unwrap();
                debug!("Got status ping: {:?}", statusping);
                let mut statuspong = StatusPong::new();
                statuspong.payload = statusping.payload;
                statuspong.write(&mut self.stream).await.unwrap();
                debug!("Sending status pong: {:?}", statuspong);
                self.state = NetworkClientState::Disconnected;
            }
            NetworkClientState::Login => {
                let (_packet_length, _packet_id) =
                    read_packet_header(&mut self.stream).await.unwrap();
                let loginstart = LoginStart::read(&mut self.stream).await.unwrap();
                debug!("{:?}", loginstart);
                // Offline mode skips encryption and compression.
                // TODO: Encryption and compression
                let mut loginsuccess = LoginSuccess::new();
                // We're in offline mode, so this is a temporary uuid.
                // TODO: Get uuid and username from Mojang servers.
                loginsuccess.uuid = "00000000-0000-3000-0000-000000000000".into();
                loginsuccess.username = loginstart.player_name;
                loginsuccess.write(&mut self.stream).await.unwrap();
                debug!("{:?}", loginsuccess);
                self.uuid = Some(loginsuccess.uuid.clone().into());
                self.username = Some(loginsuccess.username.clone().into());
                self.state = NetworkClientState::Play;
                let joingame = JoinGame::new();
                // TODO: Fill out `joingame` with actual information.
                joingame.write(&mut self.stream).await.unwrap();
                debug!("{:?}", joingame);
                let (_packet_length, _packet_id) =
                    read_packet_header(&mut self.stream).await.unwrap();
                let clientsettings = ClientSettings::read(&mut self.stream).await.unwrap();
                // TODO: Actualy use client settings.
                debug!("{:?}", clientsettings);

                // All good up to here.

                let helditemchange = HeldItemChange::new();
                // TODO: Retrieve selected slot from storage.
                helditemchange.write(&mut self.stream).await.unwrap();
                debug!("{:?}", helditemchange);
                // TODO: S->C Declare Recipes (1.16?)
                // TODO: S->C Tags (1.16?)
                // TODO: S->C Entity Status (optional?)
                // TODO: S->C Declare Commands (1.16?)
                // TODO: S->C Unlock Recipes (1.16?)
                // TODO: S->C Player Position and Look
                let playerpositionandlook = PlayerPositionAndLook::new();
                // TODO: Retrieve player position from storage.
                playerpositionandlook.write(&mut self.stream).await.unwrap();
                debug!("{:?}", playerpositionandlook);
                // TODO: S->C Player Info (Add Player action) (1.16?)
                // TODO: S->C Player Info (Update latency action) (1.16?)
                // TODO: S->C Update View Position (1.16?)
                // TODO: S->C Update Light (1.16?)
                // TODO: S->C Chunk Data
                // TODO: S->C World Border
                // TODO: S->C Spawn Position
                let spawnposition = SpawnPosition::new();
                spawnposition.write(&mut self.stream).await.unwrap();
                debug!("{:?}", spawnposition);
                // Send initial keep alive.
                self.keep_alive().await;
                // TODO: S->C Player Position and Look
                // TODO: C->S Teleport Confirm
                // TODO: C->S Player Position and Look
                // TODO: C->S Client Status
                // TODO: S->C inventories, entities, etc.
            }
            NetworkClientState::Play => {
                if self.last_keep_alive.elapsed() > Duration::from_secs(10) {
                    self.keep_alive().await;
                }
            }
            NetworkClientState::Disconnected => {
                self.connected = false;
            }
        }
    }

    /// Send a keep alive packet to the client.
    async fn keep_alive(&mut self) {
        // Keep alive ping to client.
        let clientboundkeepalive = KeepAlivePing::new();
        clientboundkeepalive.write(&mut self.stream).await.unwrap();
        debug!("{:?}", clientboundkeepalive);
        // Keep alive pong to server.
        let (_packet_length, _packet_id) = read_packet_header(&mut self.stream).await.unwrap();
        let serverboundkeepalive = KeepAlivePong::read(&mut self.stream).await.unwrap();
        debug!("{:?}", serverboundkeepalive);
        self.last_keep_alive = Instant::now();
    }
}
