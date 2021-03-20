/// Internal messaging for the server.
pub mod messages;
/// Put the network client struct in its own file.
pub mod net;

use crate::entity::player::Player;
use log::*;
use messages::*;
use net::*;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use tokio::net::{TcpListener, ToSocketAddrs};

/// The struct containing all the data and running all the updates.
pub struct Server {
    network_clients: Vec<NetworkClient>,
    network_receiver: Receiver<NetworkClient>,
    message_receiver: Receiver<ServerboundMessage>,
    // message_sender: Bus<BroadcastMessage>,
    pub players: Vec<Player>,
}
impl Server {
    pub fn new<A: 'static + ToSocketAddrs + Send>(addr: A) -> Server {
        let (network_client_tx, network_client_rx) = mpsc::channel();
        let (serverbound_message_tx, serverbound_message_rx) = mpsc::channel();
        // let mut broadcast_message_tx = Bus::new(1_000); // Hold up to 1,000 messages in the queue.
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
                        // broadcast_message_tx.add_rx(),
                    ))
                    .expect("Network receiver disconnected");
                id += 1;
            }
        });
        info!("Network server started!");
        Server {
            network_receiver: network_client_rx,
            network_clients: vec![],
            message_receiver: serverbound_message_rx,
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
        self.broadcast_message(BroadcastMessage::Disconnect(
            "The server is shutting down".into(),
        ))
        .await;
    }

    /// Update the network server.
    ///
    /// Update each client in `self.network_clients`.
    async fn update_network(&mut self) -> tokio::io::Result<()> {
        // Read new clients from the network.
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
        // Count the number of players in the Play state.
        let num_players = self.network_clients.iter().fold(0, |acc, nc| {
            if nc.state == NetworkClientState::Play {
                acc + 1
            } else {
                acc
            }
        });
        // Update each client, disconnecting those with errors.
        for client in self.network_clients.iter_mut() {
            if client.update(num_players).await.is_err() {
                client.force_disconnect();
            }
        }
        // Remove disconnected clients.
        self.network_clients
            .retain(|nc| nc.state != NetworkClientState::Disconnected);
        // Read new messages from the clients.
        loop {
            match self.message_receiver.try_recv() {
                Ok(message) => match message {
                    ServerboundMessage::Chat(msg) => {
                        self.broadcast_message(BroadcastMessage::Chat(msg)).await;
                    }
                    ServerboundMessage::PlayerJoin(_uuid, username) => {
                        self.broadcast_message(BroadcastMessage::Chat(format!(
                            "Welcome {} to the server!",
                            username
                        )))
                        .await;
                    }
                },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("Message sender disconnected"),
            }
        }
        Ok(())
    }

    pub async fn broadcast_message(&mut self, message: BroadcastMessage) {
        let mut v = Vec::new();
        for client in self.network_clients.iter_mut() {
            v.push(client.handle_broadcast_message(message.clone()));
        }
        futures::future::join_all(v).await;
    }

    /// Update the game server.
    ///
    /// Start by updating the network.
    pub async fn update(&mut self) -> tokio::io::Result<()> {
        self.update_network().await?;
        Ok(())
    }
}
