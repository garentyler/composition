/// Put the network client struct in its own file.
pub mod net;

use crate::entity::player::Player;
use log::info;
use net::*;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use tokio::net::{TcpListener, ToSocketAddrs};

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
