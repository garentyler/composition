use crate::{
    error::{Error, Result},
    net::{NetworkClient, NetworkClientState},
    server::ServerHandle,
};
use composition_protocol::{
    packets::{
        clientbound::{CS00StatusResponse, CS01PingResponse},
        serverbound::{SH00Handshake, SS00StatusRequest, SS01PingRequest},
        Packet,
    },
    prelude::*,
};
use std::sync::Arc;
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::{mpsc, oneshot, oneshot::Sender, Mutex, RwLock},
};
use tokio_util::sync::CancellationToken;
use tracing::trace;

#[derive(Debug)]
pub struct NetworkServer {
    command_channel: (
        mpsc::Sender<NetworkServerCommand>,
        Mutex<mpsc::Receiver<NetworkServerCommand>>,
    ),
    server: ServerHandle,
    running: CancellationToken,
    clients: Arc<RwLock<Vec<NetworkClient>>>,
}
impl NetworkServer {
    pub async fn new<A: ToSocketAddrs>(
        bind_address: A,
        server: ServerHandle,
    ) -> Result<NetworkServer> {
        // TODO: Test different channel sizes, 256 was chosen arbitrarily
        let (command_sender, command_receiver) = mpsc::channel(256);
        let command_receiver = Mutex::new(command_receiver);

        let network_server = NetworkServer {
            command_channel: (command_sender, command_receiver),
            running: CancellationToken::new(),
            clients: Arc::new(RwLock::new(Vec::new())),
            server,
        };

        let listener = TcpListener::bind(bind_address)
            .await
            .map_err(|_| Error::Bind)?;
        let ns = network_server.handle();
        let c = network_server.clients.clone();
        tokio::spawn(async move {
            let mut client_id = 0u128;
            loop {
                tokio::select! {
                    _ = ns.stopped() => break,
                    result = listener.accept() => {
                        if let Ok((stream, _)) = result {
                            trace!("Got client (id {})", client_id);
                            let client = NetworkClient::new(client_id, stream);
                            c.write().await.push(client);
                            client_id += 1;
                        } else {
                            trace!("Failed to accept client");
                        }
                    }
                }
            }
        });

        Ok(network_server)
    }

    /// Get a handle to the network server that can send commands asynchronously.
    pub fn handle(&self) -> NetworkServerHandle {
        NetworkServerHandle {
            command_sender: self.command_channel.0.clone(),
            running: self.running.clone(),
        }
    }
    /// Process commands until completion.
    /// Can be interrupted by cancelling `self.running`.
    pub async fn run(self) {
        let network_server = Arc::new(self);

        let mut tasks = Vec::new();
        loop {
            let mut command_receiver = network_server.command_channel.1.lock().await;

            tokio::select! {
                _ = network_server.running.cancelled() => break,
                _ = network_server.update() => {},
                command = command_receiver.recv() => {
                    let Some(command) = command else { break };
                    let network_server = network_server.clone();
                    tasks.push(tokio::spawn(async move {
                        match command {
                            NetworkServerCommand::SendPacket { packet, client_id, responder } => responder.send(network_server.send_packet(client_id, packet).await).unwrap(),
                            NetworkServerCommand::ReadPacket { client_id, responder } => responder.send(network_server.read_packet(client_id).await).unwrap(),
                            NetworkServerCommand::GetOnlinePlayerCount { responder } => responder.send(network_server.online_player_count().await).unwrap(),
                        }
                    }));
                }
            }

            tasks.retain(|task| !task.is_finished());
        }

        // Finish executing any server commands.
        futures::future::join_all(tasks).await;
        // This shouldn't fail - we just merged all of the other tasks that have a reference to `network_server`.
        let network_server = Arc::into_inner(network_server).unwrap();
        network_server.shutdown().await;
    }
    /// Try to update each client, returning updates.
    async fn update(&self) {
        let mut clients = self.clients.write().await;

        // Use tokio::spawn so that updates will process in parallel.
        let handles: Vec<_> = clients
            .drain(..)
            .zip(std::iter::repeat(self.handle()))
            .map(|(mut client, server)| {
                tokio::spawn(async move {
                    let result = Self::update_client(server, &mut client).await;
                    Result::Ok((client, result))
                })
            })
            .collect();

        // Join the tasks and parse the result of the update.
        let c = futures::future::join_all(handles).await;
        for client in c.into_iter() {
            let Ok(Ok(client)) = client else {
                continue;
            };
            let (mut client, update_result) = client;

            match update_result {
                // The update went ok, add back to the client.
                Ok(()) => clients.push(client),
                Err(Error::Protocol(e)) => client.disconnect(e.to_disconnection_reason()).await,
                Err(_) => todo!(),
            }
        }
    }
    async fn update_client(server: NetworkServerHandle, client: &mut NetworkClient) -> Result<()> {
        // TODO: Use handles to the world and server to compute updates.

        match client.state {
            NetworkClientState::Handshake => {
                // Try to read a handshake packet from the client.
                let handshake = client
                    .read_typed_packet::<SH00Handshake>()
                    .await?
                    .map_err(|_| Error::Protocol(composition_protocol::Error::UnexpectedPacket))?;

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
                        .disconnect(Some(Chat::basic("Received invalid SH00Handshake packet")))
                        .await;
                }
            }
            // Status !received_request: Read SS00StatusRequest and respond with CS00StatusResponse
            NetworkClientState::Status {
                received_request,
                received_ping,
            } if !received_request => {
                let _status_request = client
                    .read_typed_packet::<SS00StatusRequest>()
                    .await?
                    .map_err(|_| Error::Protocol(composition_protocol::Error::UnexpectedPacket))?;
                let config = crate::Config::instance();
                use base64::Engine;
                client.send_packet(CS00StatusResponse {
                    response: serde_json::json!({
                        "version": {
                            "name": config.game_version,
                            "protocol": config.protocol_version
                        },
                        "players": {
                            "max": config.max_players,
                            "online": server.online_player_count().await,
                            "sample": []
                        },
                        "description": {
                            "text": config.motd
                        },
                        "favicon": format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD_NO_PAD.encode(&config.server_icon_bytes)),
                        "enforcesSecureChat": true
                    }),
                }).await?;
                client.state = NetworkClientState::Status {
                    received_request: true,
                    received_ping,
                };
            }
            // Status !received_ping: Read SS01PingRequest and respond with CS01PingResponse
            NetworkClientState::Status { received_ping, .. } if !received_ping => {
                let ping = client
                    .read_typed_packet::<SS01PingRequest>()
                    .await?
                    .map_err(|_| Error::Protocol(composition_protocol::Error::UnexpectedPacket))?;
                client
                    .send_packet(CS01PingResponse {
                        payload: ping.payload,
                    })
                    .await?;
                client.state = NetworkClientState::Disconnected;
            }
            NetworkClientState::Status { .. } => unreachable!(),
            NetworkClientState::Login { .. } => unimplemented!(),
            NetworkClientState::Play => unimplemented!(),
            NetworkClientState::Disconnected => {}
        }
        Ok(())
    }
    async fn shutdown(self) {
        // TODO: Send "The server is shutting down." disconnect packets.
    }

    async fn read_packet(&self, client_id: u128) -> Result<Packet> {
        let clients = self.clients.read().await;
        let Some(client) = clients.iter().find(|client| client_id == client.id) else {
            return Err(Error::Message(format!("could not find a client with id {}", client_id)));
        };
        client.read_packet().await
    }
    async fn send_packet(&self, client_id: u128, packet: Packet) -> Result<()> {
        let clients = self.clients.write().await;
        let Some(client) = clients.iter().find(|client| client_id == client.id) else {
            return Err(Error::Message(format!("could not find a client with id {}", client_id)));
        };
        client.send_packet(packet).await
    }
    async fn online_player_count(&self) -> usize {
        let clients = self.clients.read().await;
        clients
            .iter()
            .filter(|client| matches!(client.state, NetworkClientState::Play))
            .count()
    }
}

#[derive(Debug)]
pub enum NetworkServerCommand {
    SendPacket {
        packet: Packet,
        client_id: u128,
        responder: Sender<Result<()>>,
    },
    ReadPacket {
        client_id: u128,
        responder: Sender<Result<Packet>>,
    },
    GetOnlinePlayerCount {
        responder: Sender<usize>,
    },
}

#[derive(Clone, Debug)]
pub struct NetworkServerHandle {
    command_sender: mpsc::Sender<NetworkServerCommand>,
    running: CancellationToken,
}
impl NetworkServerHandle {
    pub async fn read_packet(&self, client_id: u128) -> Result<Packet> {
        let (responder, response) = oneshot::channel();
        self.command_sender
            .send(NetworkServerCommand::ReadPacket {
                client_id,
                responder,
            })
            .await
            .unwrap();
        response.await.unwrap()
    }
    pub async fn send_packet<P: Into<Packet>>(&self, client_id: u128, packet: P) -> Result<()> {
        let (responder, response) = oneshot::channel();
        self.command_sender
            .send(NetworkServerCommand::SendPacket {
                packet: packet.into(),
                client_id,
                responder,
            })
            .await
            .unwrap();
        response.await.unwrap()
    }
    pub async fn online_player_count(&self) -> usize {
        let (responder, response) = oneshot::channel();
        self.command_sender
            .send(NetworkServerCommand::GetOnlinePlayerCount { responder })
            .await
            .unwrap();
        response.await.unwrap()
    }

    pub fn stop(&self) {
        self.running.cancel()
    }
    pub async fn stopped(&self) {
        let _ = self.running.cancelled().await;
    }
}
