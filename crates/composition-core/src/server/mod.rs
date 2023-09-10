use crate::{
    error::Result,
    net::{NetworkServer, NetworkServerHandle},
    world::{World, WorldHandle},
};
use std::sync::Arc;
use tokio::{
    net::ToSocketAddrs,
    sync::{mpsc, oneshot, Mutex},
};
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Debug)]
pub struct Server {
    world: Option<WorldHandle>,
    network: Option<NetworkServerHandle>,
    command_channel: (
        mpsc::Sender<ServerCommand>,
        Mutex<mpsc::Receiver<ServerCommand>>,
    ),
    running: CancellationToken,
    shutdown_sender: oneshot::Sender<()>,
}
impl Server {
    pub async fn new<A: ToSocketAddrs>(bind_address: A) -> Result<Server> {
        // TODO: Test different channel sizes, 256 was chosen arbitrarily
        let (command_sender, command_receiver) = mpsc::channel(256);
        let command_receiver = Mutex::new(command_receiver);
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();

        let mut server = Server {
            world: None,
            network: None,
            command_channel: (command_sender, command_receiver),
            running: CancellationToken::new(),
            shutdown_sender,
        };

        let network_server = NetworkServer::new(bind_address, server.handle()).await?;
        let world = World::new("world", 0, None);

        server.network = Some(network_server.handle());
        server.world = Some(world.handle());

        tokio::spawn(async move {
            world.run().await;
        });
        tokio::spawn(async move {
            network_server.run().await;
        });

        let c = server.running.clone();
        tokio::spawn(async move {
            let _ = tokio::signal::ctrl_c().await;
            info!("Ctrl-c received, shutting down...");
            c.cancel();
            match tokio::time::timeout(std::time::Duration::from_secs(10), shutdown_receiver).await
            {
                Ok(Ok(())) => {
                    // The server shutdown successfully.
                }
                Ok(Err(_)) => {
                    // The server couldn't send a shutdown message, meaning that the main thread panicked.
                    tracing::error!(
                        "Server internal shutdown sender disconnected, forcing shutdown."
                    );
                    std::process::exit(1);
                }
                Err(_) => {
                    tracing::error!("Couldn't shutdown gracefully, forcing it.");
                    std::process::exit(1);
                }
            };
        });

        Ok(server)
    }
    /// Get a handle to the server.
    pub fn handle(&self) -> ServerHandle {
        ServerHandle {
            command_sender: self.command_channel.0.clone(),
            running: self.running.clone(),
        }
    }
    /// Process commands until completion.
    /// Can be interrupted by cancelling `self.running`.
    pub async fn run(self) {
        let server = Arc::new(self);

        let mut tasks = Vec::new();
        loop {
            let mut command_receiver = server.command_channel.1.lock().await;

            tokio::select! {
                _ = server.running.cancelled() => break,
                command = command_receiver.recv() => {
                    let Some(command) = command else { break };
                    // let server = server.clone();
                    tasks.push(tokio::spawn(async move {
                        match command {
                            // TODO: Implement server commands.
                        }
                    }));
                }
            }

            tasks.retain(|task| !task.is_finished());
        }

        // Finish executing any server commands.
        futures::future::join_all(tasks).await;
        // This shouldn't fail - we just merged all of the other tasks that have a reference to `server`.
        let server = Arc::into_inner(server).unwrap();
        server.shutdown().await;
    }
    /// Run shutdown procedures.
    async fn shutdown(self) {
        let world = self.world.as_ref().unwrap();
        let network = self.network.as_ref().unwrap();
        world.stop();
        network.stop();
        let _ = futures::future::join(world.stopped(), network.stopped()).await;
        self.shutdown_sender.send(()).unwrap();
    }
}

#[derive(Debug)]
pub enum ServerCommand {}

#[derive(Debug, Clone)]
pub struct ServerHandle {
    command_sender: mpsc::Sender<ServerCommand>,
    running: CancellationToken,
}
impl ServerHandle {
    pub fn stop(&self) {
        self.running.cancel();
    }
    pub async fn stopped(&self) {
        let _ = self.running.cancelled().await;
    }
}

// use crate::config::Config;
// use crate::error::Result;
// use crate::net::server::NetworkServerHandle;
// use crate::net::{NetworkClient, NetworkClientState};
// use crate::world::WorldHandle;
// use composition_protocol::ClientState;
// use std::sync::Arc;
// use tokio::net::{TcpListener, ToSocketAddrs};
// use tokio::{sync::RwLock, task::JoinHandle};
// use tokio_util::sync::CancellationToken;
// use tracing::{error, info, trace};
//
// /// The main state and logic of the program.
// #[derive(Debug)]
// pub struct Server {
//     clients: Arc<RwLock<Vec<NetworkClient>>>,
//     net_tasks_handle: JoinHandle<()>,
// }
// impl Server {
//     #[tracing::instrument]
//     pub async fn new<A: 'static + ToSocketAddrs + Send + std::fmt::Debug>(
//         bind_address: A,
//     ) -> (Server, CancellationToken) {
//         trace!("Server::new()");

//         let running = CancellationToken::new();
//         let clients = Arc::new(RwLock::new(vec![]));
//         let net_tasks_handle = tokio::spawn(Self::create_network_tasks(
//             bind_address,
//             clients.clone(),
//             running.clone(),
//         ));

//         let server = Server {
//             clients,
//             net_tasks_handle,
//         };

//         // let (shutdown_tx, shutdown_rx) = oneshot::channel();
//         let r = running.clone();
//         tokio::spawn(async move {
//             tokio::signal::ctrl_c().await.unwrap();
//             info!("Ctrl-C received, shutting down");
//             r.cancel();
//             // shutdown_tx.send(()).unwrap();
//         });

//         (server, running)
//     }
//     #[tracing::instrument]
//     async fn create_network_tasks<A: 'static + ToSocketAddrs + Send + std::fmt::Debug>(
//         bind_address: A,
//         network_clients: Arc<RwLock<Vec<NetworkClient>>>,
//         running: CancellationToken,
//     ) {
//         // Start a task to receive new clients.
//         trace!("Creating listener task");
//         let nc = network_clients.clone();
//         let r = running.clone();
//         let listener_task = tokio::spawn(async move {
//             trace!("Listener task created");
//             let Ok(listener) = TcpListener::bind(bind_address).await else {
//                 error!("Could not bind to given address, shutting down.");
//                 std::process::exit(1);
//             };

//             let mut client_id = 0u128;
//             loop {
//                 tokio::select! {
//                     _ = r.cancelled() => {
//                         trace!("Listener task received shutdown");
//                         break;
//                     }
//                     result = listener.accept() => {
//                         if let Ok((stream, _)) = result {
//                             trace!("Listener task got client (id {})", client_id);
//                             nc.write().await.push(NetworkClient::new(client_id, stream));
//                             client_id += 1;
//                         } else {
//                             trace!("Listener task failed to accept client");
//                         }
//                     }
//                 }
//             }
//         });

//         // Start a task to update existing clients' packet queues.
//         trace!("Creating network task");
//         let nc = network_clients.clone();
//         let r = running.clone();
//         let packet_task = tokio::spawn(async move {
//             trace!("Network task created");
//             loop {
//                 // Start tasks to read/write to clients concurrently.
//                 tokio::select! {
//                     _ = r.cancelled() => {
//                         trace!("Network task received shutdown");
//                         break;
//                     }
//                     mut nc = nc.write() => {
//                         trace!("Network task updating clients");
//                         let tasks: Vec<JoinHandle<NetworkClient>> = nc
//                             .drain(..)
//                             .map(|mut client: NetworkClient| {
//                                 tokio::spawn(async move {
//                                     let _ = client.read_packets().await;
//                                     if client.send_queued_packets().await.is_err() {
//                                         client
//                                             .disconnect(Some(serde_json::json!({ "text": "Error writing packets." })))
//                                             .await;
//                                     }
//                                     client
//                                 })
//                             })
//                             .collect();
//                         *nc = Vec::with_capacity(tasks.len());
//                         for task in tasks {
//                             nc.push(task.await.unwrap());
//                         }
//                         trace!("Network task updated clients");
//                     }
//                 }
//             }
//         });

//         // Start a task to remove disconnected clients.
//         trace!("Creating disconnection task");
//         let nc = network_clients.clone();
//         let r = running.clone();
//         let disconnection_task = tokio::spawn(async move {
//             trace!("Disconnection task created");
//             loop {
//                 tokio::select! {
//                     _ = r.cancelled() => {
//                         trace!("Disconnection task received shutdown");
//                         break;
//                     }
//                     mut nc = nc.write() => {
//                         let before = nc.len();
//                         nc.retain(|client| client.state != NetworkClientState::Disconnected);
//                         let after = nc.len();
//                         trace!("Disconnection task removed {} clients", before - after);
//                     }
//                 }
//             }
//         });

//         // Join the tasks on shutdown.
//         listener_task.await.expect("Listener task crashed");
//         packet_task.await.expect("Packet task crashed");
//         disconnection_task
//             .await
//             .expect("Disconnection task crashed");
//     }
//     #[tracing::instrument]
//     pub async fn update(&mut self) -> Result<()> {
//         trace!("Server.update()");

//         let mut clients = self.clients.write().await;

//         // Handle packets from the clients.
//         let online_players = clients
//             .iter()
//             .filter(|client| matches!(client.state, NetworkClientState::Play))
//             .count();
//         'clients: for client in clients.iter_mut() {
//             use composition_protocol::packets::{clientbound::*, serverbound::*};
//             'packets: while !client.incoming_packet_queue.is_empty() {
//                 // client.read_packet()
//                 // None: The client doesn't have any more packets.
//                 // Some(Err(_)): The client read an unexpected packet. TODO: Handle this error.
//                 // Some(Ok(_)): The client read the expected packet.
//                 match client.state.clone() {
//                     NetworkClientState::Handshake => {
//                         let handshake = match client.read_packet::<SH00Handshake>() {
//                             None => continue 'packets,
//                             Some(Err(_)) => continue 'clients,
//                             Some(Ok(handshake)) => handshake,
//                         };

//                         if handshake.next_state == ClientState::Status {
//                             client.state = NetworkClientState::Status {
//                                 received_request: false,
//                                 received_ping: false,
//                             };
//                         } else if handshake.next_state == ClientState::Login {
//                             client.state = NetworkClientState::Login {
//                                 received_start: (false, None),
//                             };
//                         } else {
//                             client
//                                 .disconnect(Some(
//                                     serde_json::json!({ "text": "Received invalid SH00Handshake packet" }),
//                                 ))
//                                 .await;
//                         }
//                     }
//                     // Status !received_request: Read SS00StatusRequest and respond with CS00StatusResponse
//                     NetworkClientState::Status {
//                         received_request,
//                         received_ping,
//                     } if !received_request => {
//                         let _status_request = match client.read_packet::<SS00StatusRequest>() {
//                             None => continue 'packets,
//                             Some(Err(_)) => continue 'clients,
//                             Some(Ok(p)) => p,
//                         };
//                         client.state = NetworkClientState::Status {
//                             received_request: true,
//                             received_ping,
//                         };
//                         let config = Config::instance();
//                         use base64::Engine;
//                         client.queue_packet(CS00StatusResponse {
//                             response: serde_json::json!({
//                                 "version": {
//                                     "name": config.game_version,
//                                     "protocol": config.protocol_version
//                                 },
//                                 "players": {
//                                     "max": config.max_players,
//                                     "online": online_players,
//                                     "sample": []
//                                 },
//                                 "description": {
//                                     "text": config.motd
//                                 },
//                                 "favicon": format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD_NO_PAD.encode(&config.server_icon_bytes)),
//                                 "enforcesSecureChat": true
//                             }),
//                         });
//                     }
//                     // Status !received_ping: Read SS00StatusRequest and respond with CS00StatusResponse
//                     NetworkClientState::Status { received_ping, .. } if !received_ping => {
//                         let ping = match client.read_packet::<SS01PingRequest>() {
//                             None => continue 'packets,
//                             Some(Err(_)) => continue 'clients,
//                             Some(Ok(p)) => p,
//                         };
//                         client.queue_packet(CS01PingResponse {
//                             payload: ping.payload,
//                         });
//                         client.state = NetworkClientState::Disconnected;
//                     }
//                     NetworkClientState::Status { .. } => unreachable!(),
//                     NetworkClientState::Login { received_start, .. } if !received_start.0 => {
//                         let login_start = match client.read_packet::<SL00LoginStart>() {
//                             None => continue 'packets,
//                             Some(Err(_)) => continue 'clients,
//                             Some(Ok(p)) => p,
//                         };
//                         // TODO: Authenticate the user.
//                         // TODO: Get the user from the stored database.
//                         // TODO: Encryption/compression.
//                         client.queue_packet(CL02LoginSuccess {
//                             uuid: login_start.uuid.unwrap_or(0u128),
//                             username: login_start.name.clone(),
//                             properties: vec![],
//                         });
//                         client.state = NetworkClientState::Login {
//                             received_start: (true, Some(login_start)),
//                         };
//                     }
//                     NetworkClientState::Login { .. } => unreachable!(),
//                     NetworkClientState::Play => unimplemented!(),
//                     NetworkClientState::Disconnected => unimplemented!(),
//                 }
//                 // If continue was not
//                 break 'packets;
//             }
//         }

//         Ok(())
//     }
//     #[tracing::instrument]
//     pub async fn shutdown(self) {
//         trace!("Server.shutdown()");

//         // Close the concurrent tasks.
//         let _ = self.net_tasks_handle.await;

//         // Send disconnect messages to the clients.
//         for client in self.clients.write().await.iter_mut() {
//             client
//                 .disconnect(Some(
//                     serde_json::json!({ "text": "The server is shutting down." }),
//                 ))
//                 .await;
//         }
//     }
// }
