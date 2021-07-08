/// Internal messaging for the server.
pub mod messages;
/// Put the network client struct in its own file.
pub mod net;

use crate::entity::player::Player;
use crate::{mctypes::*, CONFIG, FAVICON};
use log::*;
use messages::*;
use net::{
    packets::{self, Packet, PacketCommon},
    *,
};
use serde_json::json;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::Duration;
use tokio::net::{TcpListener, ToSocketAddrs};

/// The struct containing all the data and running all the updates.
pub struct Server {
    network_clients: Vec<NetworkClient>,
    network_receiver: Receiver<NetworkClient>,
    pub players: Vec<Player>,
}
impl Server {
    pub fn new<A: 'static + ToSocketAddrs + Send>(addr: A) -> Server {
        let (network_client_tx, network_client_rx) = mpsc::channel();
        let (serverbound_message_tx, _serverbound_message_rx) = mpsc::channel();
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
                network_client_tx
                    .send(NetworkClient::new(
                        stream,
                        id as u128,
                        serverbound_message_tx.clone(),
                    ))
                    .expect("Network receiver disconnected");
                id += 1;
            }
        });
        info!("Network server started!");
        Server {
            network_receiver: network_client_rx,
            network_clients: vec![],
            // message_receiver: serverbound_message_rx,
            players: vec![],
        }
    }

    /// Shut down the server.
    ///
    /// Disconnects all clients.
    pub async fn shutdown(&mut self) {
        info!(
            "Server shutting down. Uptime: {:?}",
            crate::START_TIME.elapsed()
        );
        self.broadcast_message(ClientboundMessage::Disconnect(
            "The server is shutting down".into(),
        ))
        .await;
    }

    pub fn num_players(&self) -> usize {
        let mut num = 0;
        for client in &self.network_clients {
            if client.state == NetworkClientState::Play {
                num += 1;
            }
        }
        num
    }

    /// Send a `ClientboundMessage` to all connected network clients.
    pub async fn broadcast_message(&mut self, message: ClientboundMessage) {
        let mut v = Vec::new();
        for client in self.network_clients.iter_mut() {
            v.push(client.handle_message(message.clone()));
        }
        futures::future::join_all(v).await;
    }

    /// Get a client from their id.
    pub fn client_from_id(&mut self, client_id: u128) -> Option<&mut NetworkClient> {
        // Find the client based on id.
        let mut client_index = -1isize;
        for (i, c) in self.network_clients.iter().enumerate() {
            if c.id == client_id {
                client_index = i as isize;
                break;
            }
        }
        if client_index == -1 {
            return None;
        }
        Some(&mut self.network_clients[client_index as usize])
    }

    /// Update the game server.
    pub async fn update(&mut self) -> tokio::io::Result<()> {
        // Get new clients from the network listener thread.
        loop {
            match self.network_receiver.try_recv() {
                Ok(client) => {
                    info!(
                        "Got client at {}",
                        client.stream.peer_addr().expect("could not get peer addr")
                    );
                    self.network_clients.push(client)
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("network sender disconnected"),
            }
        }
        // Read new packets from each client.
        for client in self.network_clients.iter_mut() {
            // Update and ignore errors.
            if client.update().await.is_err() {
                client.force_disconnect();
            }
        }
        // Update the client and server according to each packet.
        let mut packets = vec![];
        for client in self.network_clients.iter_mut() {
            while let Some(packet) = client.read_packet() {
                packets.push((client.id, packet));
            }
        }
        for (client_id, packet) in packets {
            if self.handle_packet(client_id, packet).await.is_err() {
                self.client_from_id(client_id).unwrap().force_disconnect();
            }
        }
        // Disconnect clients when necessary.
        for client in self.network_clients.iter_mut() {
            if client.state == NetworkClientState::Disconnected {
                client.force_disconnect();
            } else if client.last_keep_alive.elapsed() > Duration::from_secs(20) {
                debug!("Disconnecting client for timing out");
                client.state = NetworkClientState::Disconnected;
                client.force_disconnect();
            }
        }
        // Remove disconnected clients.
        self.network_clients
            .retain(|nc| nc.state != NetworkClientState::Disconnected);
        Ok(())
    }

    /// Handle a packet.
    pub async fn handle_packet<P: PacketCommon>(
        &mut self,
        client_id: u128,
        packet: P,
    ) -> tokio::io::Result<()> {
        let num_players = self.num_players();
        let mut client = self.client_from_id(client_id).unwrap();
        match packet.as_packet() {
            // Handshaking.
            Packet::Handshake(handshake) => {
                if handshake.next_state == 1 {
                    client.state = NetworkClientState::Status;
                } else if handshake.next_state == 2 {
                    client.state = NetworkClientState::Login;
                } else {
                    client.state = NetworkClientState::Disconnected;
                }
                if handshake.protocol_version != 47 {
                    let mut logindisconnect = packets::LoginDisconnect::new();
                    logindisconnect.reason = MCChat {
                        text: MCString::from("Incompatible client! Server is on 1.8.9"),
                    };
                    client.send_packet(logindisconnect).await?;
                    client.state = NetworkClientState::Disconnected;
                }
            }
            // Status.
            Packet::StatusRequest(_statusrequest) => {
                let mut statusresponse = packets::StatusResponse::new();
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
                client.send_packet(statusresponse).await?;
            }
            Packet::StatusPing(statusping) => {
                let mut statuspong = packets::StatusPong::new();
                statuspong.payload = statusping.payload;
                client.send_packet(statuspong).await?;
            }
            // Login.
            Packet::LoginStart(loginstart) => {
                client.player = Some(crate::entity::player::Player::new());
                let player = client.player.as_mut().unwrap();
                *player.username_mut() = loginstart.player_name.into();
                // Offline mode skips encryption and compression.
                // TODO: Encryption and compression
                let mut loginsuccess = packets::LoginSuccess::new();
                // We're in offline mode, so this is a temporary uuid.
                // TODO: Get uuid and username from Mojang servers.
                loginsuccess.uuid = player.uuid.clone().to_hyphenated().to_string().into();
                loginsuccess.username = player.username().clone().into();
                client.send_packet(loginsuccess).await?;
                client.state = NetworkClientState::Play;
                client.send_packet(packets::JoinGame::new()).await?;
            }
            Packet::ClientSettings(_clientsettings) => {
                // TODO: Handle the packet.
                client.send_packet(packets::HeldItemChange::new()).await?;
                client
                    .send_packet(packets::ClientboundPlayerPositionAndLook::new())
                    .await?;
                client.send_packet(packets::SpawnPosition::new()).await?;
            }
            // Play.
            Packet::KeepAlivePong(_keepalivepong) => {
                // TODO: Handle the packet.
            }
            Packet::ServerboundChatMessage(chatmessage) => {
                let player_name = client.player.as_ref().unwrap().username().clone();
                info!("<{}> {}", player_name, chatmessage.text);
                self.broadcast_message(ClientboundMessage::Chat(format!(
                    "<{}> {}",
                    player_name, chatmessage.text
                )))
                .await;
                // TODO: Handle the packet.
            }
            Packet::Player(_player) => {
                // TODO: Handle the packet.
            }
            Packet::PlayerPosition(_playerposition) => {
                // TODO: Handle the packet.
            }
            Packet::PlayerLook(_playerlook) => {
                // TODO: Handle the packet.
            }
            Packet::ServerboundPlayerPositionAndLook(_playerpositionandlook) => {
                // TODO: Handle the packet.
            }
            // Other.
            _ => error!("handling unknown packet type: {:?}", packet),
        }
        Ok(())
    }
}
