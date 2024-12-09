pub mod config;
pub mod error;

use crate::net::connection::Connection;
use crate::App;
use crate::{config::Config, net::connection::ConnectionManager};
use config::ProxyConfig;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{info, trace, error, debug};
use error::{Error, NetworkError};

#[derive(Debug)]
pub struct Proxy {
    running: CancellationToken,
    connections: ConnectionManager,
    listener: JoinHandle<()>,
    upstream: Connection,
}
#[async_trait::async_trait]
impl App for Proxy {
    type Error = Error;

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
        
        // Only allow one client to join at a time.
        let connections = ConnectionManager::new(Some(1));
        let listener = connections
            .spawn_listener(bind_address, running.child_token())
            .await
            .map_err(Error::Network)?;

        let upstream_address = format!("{}:{}", config.proxy.upstream_host, config.proxy.upstream_port);
        info!("Upstream server: {}", upstream_address);
        let upstream = TcpStream::connect(upstream_address).await.map_err(Error::Io)?;
        let upstream = Connection::new_server(0, upstream);

        Ok(Proxy {
            running,
            connections,
            listener,
            upstream,
        })
    }
    #[tracing::instrument]
    async fn update(&mut self) -> Result<(), Self::Error> {
        let _ = self.connections.update().await.map_err(Error::Network)?;

        let Some(client) = self.connections.clients_mut().take(1).next() else {
            return Ok(());
        };

        let mut client_parsing_error = false;
        
        // At the same time, try to read packets from the server and client.
        // Forward the packet onto the other.
        tokio::select! {
            packet = client.read_packet() => {
                if let Some(packet) = packet {
                    match packet {
                        Ok(packet) => {
                            trace!("Got packet from client: {:?}", packet);
                            self.upstream.send_packet(packet).await.map_err(Error::Network)?;
                        }
                        Err(NetworkError::Parsing) => {
                            debug!("Got invalid data from client (id {})", client.id());
                            client_parsing_error = true;
                        }
                        Err(e) => return Err(Error::Network(e)),
                    }
                }
            }
            packet = self.upstream.read_packet() => {
                if let Some(packet) = packet {
                    match packet {
                        Ok(packet) => {
                            trace!("Got packet from upstream: {:?}", packet);
                            client.send_packet(packet).await.map_err(Error::Network)?;
                        }
                        Err(NetworkError::Parsing) => {
                            error!("Got invalid data from upstream");
                            return Err(Error::Network(NetworkError::Parsing));
                        },
                        Err(e) => return Err(Error::Network(e)),
                    }
                }
            }
        }

        if client_parsing_error {
            let id = client.id();
            // Drop the &mut Connection
            let _ = client;
            let _ = self.connections.disconnect(id, Some(serde_json::json!({ "text": "Received malformed data." }))).await;
        }

        Ok(())
    }
    #[tracing::instrument]
    async fn shutdown(self) -> Result<(), Self::Error> {
        // Ensure any child tasks have been shut down.
        self.running.cancel();

        let _ = self.listener.await.map_err(Error::Task)?;
        let _ = self.connections.shutdown(None).await.map_err(Error::Network)?;

        Ok(())
    }
}
