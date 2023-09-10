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
