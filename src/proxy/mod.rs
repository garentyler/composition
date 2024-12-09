pub mod config;

use crate::App;
use crate::{config::Config, net::connection::ConnectionManager};
use config::ProxyConfig;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Debug)]
pub struct Proxy {
    running: CancellationToken,
    connections: ConnectionManager,
    listener: JoinHandle<()>,
}
#[async_trait::async_trait]
impl App for Proxy {
    type Error = ();

    fn startup_message() -> String {
        let config = Config::instance();
        format!(
            "Starting {} on port {}",
            ProxyConfig::default().version,
            config.proxy.port
        )
    }
    #[tracing::instrument]
    async fn new(running: CancellationToken) -> Result<Self, Self::Error> {
        let config = Config::instance();
        let bind_address = format!("0.0.0.0:{}", config.proxy.port);

        let connections = ConnectionManager::new();
        let listener = connections
            .spawn_listener(bind_address, running.child_token())
            .await
            .map_err(|_| ())?;

        info!(
            "Upstream server: {}:{}",
            config.proxy.upstream_host, config.proxy.upstream_port
        );

        Ok(Proxy {
            running,
            connections,
            listener,
        })
    }
    #[tracing::instrument]
    async fn update(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
    #[tracing::instrument]
    async fn shutdown(self) -> Result<(), Self::Error> {
        // Ensure any child tasks have been shut down.
        self.running.cancel();

        let _ = self.listener.await.map_err(|_| ())?;
        let _ = self.connections.shutdown(None).await.map_err(|_| ())?;

        Ok(())
    }
}
