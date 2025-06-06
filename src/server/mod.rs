/// Server-specific configuration.
pub mod config;
/// When managing the server encounters errors.
pub mod error;

use crate::{
    config::Config,
    net::connection::{DownstreamConnectionManager, DownstreamConnectionState},
    server::{config::ServerConfig, error::Error},
    App,
};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

/// The main state and logic of the program.
#[derive(Debug)]
pub struct Server {
    running: CancellationToken,
    connections: DownstreamConnectionManager,
    listener: JoinHandle<()>,
}
#[async_trait::async_trait]
impl App for Server {
    type Error = Error;

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

        // No limit on connections.
        let connections = DownstreamConnectionManager::new(None);
        let listener = connections
            .spawn_listener(bind_address, running.child_token())
            .await
            .map_err(Error::Network)?;

        Ok(Server {
            running,
            connections,
            listener,
        })
    }
    #[tracing::instrument]
    async fn update(&mut self) -> Result<(), Self::Error> {
        let online_player_count = self
            .connections
            .clients()
            .filter(|c| matches!(c.client_state(), DownstreamConnectionState::Play))
            .count();

        // Receive new connections and remove disconnected ones.
        self.connections.update().await?;

        // Read packets from each connection.
        // Handle handshake connections.
        let _ = futures::future::join_all(
            self.connections
                .clients_mut()
                .filter(|c| matches!(c.client_state(), DownstreamConnectionState::Handshake))
                .map(|c| c.handle_handshake()),
        )
        .await;

        // Handle status connections.
        let _ = futures::future::join_all(
            self.connections
                .clients_mut()
                .filter(|c| matches!(c.client_state(), DownstreamConnectionState::StatusRequest))
                .map(|c| c.handle_status_ping(online_player_count)),
        )
        .await;

        // Handle login connections.
        // Handle play connection packets.
        // Process world updates.
        // Send out play connection updates.

        Ok(())
    }
    #[tracing::instrument]
    async fn shutdown(self) -> Result<(), Self::Error> {
        // Ensure any child tasks have been shut down.
        self.running.cancel();

        let _ = self.listener.await.map_err(Error::Task)?;
        let _ = self
            .connections
            .shutdown(Some(
                serde_json::json!({ "text": "The server is shutting down." }),
            ))
            .await
            .map_err(Error::Network)?;

        Ok(())
    }
}
