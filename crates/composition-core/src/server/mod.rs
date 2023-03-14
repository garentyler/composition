use crate::{
    net::{mctypes::Position, *},
    prelude::*,
};
use std::time::Duration;
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::mpsc::{self, error::TryRecvError, UnboundedReceiver},
};

#[derive(Clone, Debug, PartialEq)]
pub enum ServerError {}

pub struct Server {
    network_client_receiver: UnboundedReceiver<NetworkClient>,
    clients: Vec<NetworkClient>,
}
impl Server {
    pub async fn new<A: 'static + ToSocketAddrs + Send>(bind_address: A) -> Server {
        let (client_tx, client_rx) = mpsc::unbounded_channel();
        tokio::task::spawn(async move {
            let listener = TcpListener::bind(bind_address)
                .await
                .expect("Could not bind to given address");
            let mut id = 0u128;
            loop {
                trace!("Server accepting new client");
                match listener.accept().await {
                    Ok((socket, addr)) => {
                        let _ = client_tx.send(NetworkClient::new(id, socket));
                        debug!("Connected client {} at {:?}", id, addr);
                        id += 1;
                    }
                    Err(_) => break,
                }
            }
        });
        Server {
            network_client_receiver: client_rx,
            clients: vec![],
        }
    }
    pub async fn update(&mut self) -> Result<(), ServerError> {
        trace!("Server.update()");
        // Read new clients from the receiver
        loop {
            match self.network_client_receiver.try_recv() {
                Ok(client) => self.clients.push(client),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("Client sender disconnected"),
            }
        }
        // Remove disconnected clients.
        let mut i = 0;
        while i < self.clients.len() {
            if self.clients[i].state == NetworkClientState::Disconnected {
                debug!("Removed client {}", self.clients[i].id);
                self.clients.remove(i);
            } else if self.clients[i].last_data_time.elapsed() > Duration::from_secs(10) {
                debug!("Client {} timed out", self.clients[i].id);
                self.clients[i].disconnect(None).await;
            } else {
                i += 1;
            }
        }
        // Read data and try to parse packets from each client.
        for client in self.clients.iter_mut() {
            if client.state == NetworkClientState::Disconnected {
                continue;
            }
            let _ = client.read_data().await;
            'packet: loop {
                match client.read_packet() {
                    Ok(_) => {}
                    Err(ParseError::NotEnoughData) => break 'packet,
                    Err(_) => {}
                }
            }
        }
        // Handle each packet for each player.
        'client: for i in 0..self.clients.len() {
            while let Some(packet) = self.clients[i].packet_queue.pop_front() {
                if self.handle_packet(i, packet).await.is_err() {
                    continue 'client;
                }
            }
        }
        // Handle general world updates.
        // Send out packets to each client.

        Ok(())
    }
    pub async fn handle_packet(&mut self, client_index: usize, packet: Packet) -> Result<(), ()> {
        use Packet::*;
        trace!("Server.handle_packet()");
        debug!("Handling packet {:?}", packet);
        let mut current_players = 0;
        for client in &self.clients {
            if client.state == NetworkClientState::Play {
                current_players += 1;
            }
        }
        let client = &mut self.clients[client_index];
        match packet {
            SH00Handshake {
                protocol_version,
                server_address: _,
                server_port: _,
                next_state,
            } => {
                if protocol_version != CONFIG.protocol_version
                    && next_state == NetworkClientState::Login
                {
                    debug!(
                        "Disconnecting client {} for mismatched protocols: {} (expected {})",
                        client.id, protocol_version, CONFIG.protocol_version
                    );
                    client.disconnect(None).await;
                    return Err(());
                }
                client.state = next_state;
            }
            SS00Request => {
                let _ = client
                    .send_packet(CS00Response {
                        version_name: CONFIG.game_version.clone(),
                        protocol_version: CONFIG.protocol_version,
                        max_players: CONFIG.max_players,
                        current_players,
                        description: json!({
                            "text": CONFIG.motd
                        }),
                    })
                    .await;
            }
            SS01Ping { payload } => {
                let _ = client.send_packet(CS01Pong { payload }).await;
                debug!("Disconnecting client {}, SLP completed", client.id);
                client.disconnect(None).await;
            }
            SL00LoginStart { username } => {
                debug!(
                    "Client {} is connecting with username {}",
                    client.id, username
                );
                if current_players >= CONFIG.max_players {
                    client
                        .disconnect(Some(json!({ "text": "Server full!" })))
                        .await;
                }
                // TODO: Authentication
                // TODO: Encryption
                // TODO: Compression
                let _ = client
                    .send_packet(CL02LoginSuccess {
                        uuid: client.id,
                        username,
                    })
                    .await;
                client.state = NetworkClientState::Play;
                // Log them in.
                let _ = client
                    .send_packet(CP4BSpawnPosition {
                        location: Position::new(0, 0, 0),
                        angle: 0.0,
                    })
                    .await;
                let _ = client
                    .send_packet(CP14WindowItems {
                        window_id: 0,
                        state_id: 0,
                        slots: vec![quartz_nbt::compound! {}; 44],
                        carried_item: quartz_nbt::compound! {},
                    })
                    .await;
                let _ = client
                    .send_packet(CP38PlayerPositionAndLook {
                        x: (0.0, false),
                        y: (0.0, false),
                        z: (0.0, false),
                        yaw: (0.0, false),
                        pitch: (0.0, false),
                        teleport_id: 0,
                        dismount_vehicle: false,
                    })
                    .await;
            }
            _ => unimplemented!("Handling unknown packet"),
        }
        Ok(())
    }
    pub async fn shutdown(self) -> Result<(), ServerError> {
        trace!("Server.shutdown()");
        Ok(())
    }
}
