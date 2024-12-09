/// Server-specific configuration.
pub mod config;
/// When managing the server encounters errors.
pub mod error;
/// Network operations.
pub mod net;

use crate::config::Config;
use crate::protocol::ClientState;
use crate::App;
use config::ServerConfig;
use net::{NetworkClient, NetworkClientState};
use std::sync::Arc;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::{sync::RwLock, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::{error, trace};

/// The main state and logic of the program.
#[derive(Debug)]
pub struct Server {
    clients: Arc<RwLock<Vec<NetworkClient>>>,
    net_tasks_handle: JoinHandle<()>,
}
impl Server {
    #[tracing::instrument]
    async fn create_network_tasks<A: 'static + ToSocketAddrs + Send + std::fmt::Debug>(
        bind_address: A,
        network_clients: Arc<RwLock<Vec<NetworkClient>>>,
        running: CancellationToken,
    ) {
        // Start a task to receive new clients.
        trace!("Creating listener task");
        let nc = network_clients.clone();
        let r = running.clone();
        let listener_task = tokio::spawn(async move {
            trace!("Listener task created");
            let Ok(listener) = TcpListener::bind(bind_address).await else {
                error!("Could not bind to given address, shutting down.");
                std::process::exit(1);
            };

            let mut client_id = 0u128;
            loop {
                tokio::select! {
                    _ = r.cancelled() => {
                        trace!("Listener task received shutdown");
                        break;
                    }
                    result = listener.accept() => {
                        if let Ok((stream, _)) = result {
                            trace!("Listener task got client (id {})", client_id);
                            nc.write().await.push(NetworkClient::new(client_id, stream));
                            client_id += 1;
                        } else {
                            trace!("Listener task failed to accept client");
                        }
                    }
                }
            }
        });

        // Start a task to update existing clients' packet queues.
        trace!("Creating network task");
        let nc = network_clients.clone();
        let r = running.clone();
        let packet_task = tokio::spawn(async move {
            trace!("Network task created");
            loop {
                // Start tasks to read/write to clients concurrently.
                tokio::select! {
                    _ = r.cancelled() => {
                        trace!("Network task received shutdown");
                        break;
                    }
                    mut nc = nc.write() => {
                        trace!("Network task updating clients");
                        let tasks: Vec<JoinHandle<NetworkClient>> = nc
                            .drain(..)
                            .map(|mut client: NetworkClient| {
                                tokio::spawn(async move {
                                    let _ = client.read_packets().await;
                                    if client.send_queued_packets().await.is_err() {
                                        client
                                            .disconnect(Some(serde_json::json!({ "text": "Error writing packets." })))
                                            .await;
                                    }
                                    client
                                })
                            })
                            .collect();
                        *nc = Vec::with_capacity(tasks.len());
                        for task in tasks {
                            nc.push(task.await.unwrap());
                        }
                        trace!("Network task updated clients");
                    }
                }
            }
        });

        // Start a task to remove disconnected clients.
        trace!("Creating disconnection task");
        let nc = network_clients.clone();
        let r = running.clone();
        let disconnection_task = tokio::spawn(async move {
            trace!("Disconnection task created");
            loop {
                tokio::select! {
                    _ = r.cancelled() => {
                        trace!("Disconnection task received shutdown");
                        break;
                    }
                    mut nc = nc.write() => {
                        let before = nc.len();
                        nc.retain(|client| client.state != NetworkClientState::Disconnected);
                        let after = nc.len();
                        trace!("Disconnection task removed {} clients", before - after);
                    }
                }
            }
        });

        // Join the tasks on shutdown.
        listener_task.await.expect("Listener task crashed");
        packet_task.await.expect("Packet task crashed");
        disconnection_task
            .await
            .expect("Disconnection task crashed");
    }
}
#[async_trait::async_trait]
impl App for Server {
    type Error = error::Error;

    fn startup_message() -> String {
        let config = Config::instance();
        format!(
            "Starting {} on port {}",
            ServerConfig::default().version,
            config.server.port
        )
    }
    #[tracing::instrument]
    async fn new(running: CancellationToken) -> Result<Self, Self::Error> {
        let config = Config::instance();
        let bind_address = format!("0.0.0.0:{}", config.server.port);

        let clients = Arc::new(RwLock::new(vec![]));
        let net_tasks_handle = tokio::spawn(Self::create_network_tasks(
            bind_address,
            clients.clone(),
            running.child_token(),
        ));

        Ok(Server {
            clients,
            net_tasks_handle,
        })
    }
    #[tracing::instrument]
    async fn update(&mut self) -> Result<(), Self::Error> {
        let mut clients = self.clients.write().await;

        // Handle packets from the clients.
        let online_players = clients
            .iter()
            .filter(|client| matches!(client.state, NetworkClientState::Play))
            .count();
        'clients: for client in clients.iter_mut() {
            use crate::protocol::packets;

            'packets: while !client.incoming_packet_queue.is_empty() {
                // client.read_packet()
                // None: The client doesn't have any more packets.
                // Some(Err(_)): The client read an unexpected packet. TODO: Handle this error.
                // Some(Ok(_)): The client read the expected packet.
                match client.state.clone() {
                    NetworkClientState::Handshake => {
                        use packets::handshake::serverbound::Handshake;

                        let handshake = match client.read_packet::<Handshake>() {
                            None => continue 'packets,
                            Some(Err(_)) => continue 'clients,
                            Some(Ok(handshake)) => handshake,
                        };

                        if handshake.next_state == ClientState::Status {
                            client.state = NetworkClientState::Status {
                                received_request: false,
                                received_ping: false,
                            };
                        } else if handshake.next_state == ClientState::Login {
                            client.state = NetworkClientState::Login {
                                received_start: (false, None),
                            };
                        } else {
                            client
                                .disconnect(Some(
                                    serde_json::json!({ "text": "Received invalid SH00Handshake packet" }),
                                ))
                                .await;
                        }
                    }
                    // Status !received_request: Read SS00StatusRequest and respond with CS00StatusResponse
                    NetworkClientState::Status {
                        received_request,
                        received_ping,
                    } if !received_request => {
                        use packets::status::{
                            clientbound::StatusResponse, serverbound::StatusRequest,
                        };

                        let _status_request = match client.read_packet::<StatusRequest>() {
                            None => continue 'packets,
                            Some(Err(_)) => continue 'clients,
                            Some(Ok(p)) => p,
                        };
                        client.state = NetworkClientState::Status {
                            received_request: true,
                            received_ping,
                        };
                        let config = Config::instance();
                        use base64::Engine;
                        client.queue_packet(StatusResponse {
                            response: serde_json::json!({
                                "version": {
                                    "name": config.global.game_version,
                                    "protocol": config.global.protocol_version
                                },
                                "players": {
                                    "max": config.server.max_players,
                                    "online": online_players,
                                    "sample": []
                                },
                                "description": {
                                    "text": config.server.motd
                                },
                                "favicon": format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD_NO_PAD.encode(&config.server.server_icon_bytes)),
                                "enforcesSecureChat": true
                            }),
                        });
                    }
                    // Status !received_ping: Read SS00StatusRequest and respond with CS00StatusResponse
                    NetworkClientState::Status { received_ping, .. } if !received_ping => {
                        use packets::status::{
                            clientbound::PingResponse, serverbound::PingRequest,
                        };

                        let ping = match client.read_packet::<PingRequest>() {
                            None => continue 'packets,
                            Some(Err(_)) => continue 'clients,
                            Some(Ok(p)) => p,
                        };
                        client.queue_packet(PingResponse {
                            payload: ping.payload,
                        });
                        client.state = NetworkClientState::Disconnected;
                    }
                    NetworkClientState::Status { .. } => unreachable!(),
                    NetworkClientState::Login { received_start, .. } if !received_start.0 => {
                        use packets::login::{clientbound::*, serverbound::*};

                        let login_start = match client.read_packet::<LoginStart>() {
                            None => continue 'packets,
                            Some(Err(_)) => continue 'clients,
                            Some(Ok(p)) => p,
                        };
                        // TODO: Authenticate the user.
                        // TODO: Get the user from the stored database.
                        // TODO: Encryption/compression.
                        client.queue_packet(LoginSuccess {
                            uuid: login_start.uuid.unwrap_or(0u128),
                            username: login_start.name.clone(),
                            properties: vec![],
                        });
                        client.state = NetworkClientState::Login {
                            received_start: (true, Some(login_start)),
                        };
                    }
                    NetworkClientState::Login { .. } => unreachable!(),
                    NetworkClientState::Play => unimplemented!(),
                    NetworkClientState::Disconnected => unimplemented!(),
                }
                // If continue was not
                break 'packets;
            }
        }

        Ok(())
    }
    #[tracing::instrument]
    async fn shutdown(self) -> Result<(), Self::Error> {
        // Close the concurrent tasks.
        let _ = self.net_tasks_handle.await;

        // Send disconnect messages to the clients.
        for client in self.clients.write().await.iter_mut() {
            client
                .disconnect(Some(
                    serde_json::json!({ "text": "The server is shutting down." }),
                ))
                .await;
        }

        Ok(())
    }
}
