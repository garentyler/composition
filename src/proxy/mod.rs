pub mod config;
pub mod error;

use crate::protocol::packets::Packet;
use crate::protocol::ClientState;
use crate::App;
use crate::{
    config::Config,
    net::connection::{DownstreamConnectionManager, UpstreamConnection},
};
use config::ProxyConfig;
use error::{Error, NetworkError};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{info, trace};

#[derive(Debug)]
pub struct Proxy {
    running: CancellationToken,
    connections: DownstreamConnectionManager,
    listener: JoinHandle<()>,
    upstream_address: String,
    upstream: UpstreamConnection,
}
impl Proxy {
    pub async fn connect_upstream(upstream_address: &str) -> Result<UpstreamConnection, Error> {
        let upstream = TcpStream::connect(upstream_address)
            .await
            .map_err(Error::Io)?;

        Ok(UpstreamConnection::new(0, upstream))
    }
    pub fn rewrite_packet(packet: Packet) -> Option<Packet> {
        match packet {
            Packet::StatusResponse(mut status) => {
                let new_description = ProxyConfig::default().version.clone();
                *status
                    .response
                    .as_object_mut()
                    .unwrap()
                    .get_mut("description")
                    .unwrap() = serde_json::Value::String(new_description);
                Some(Packet::StatusResponse(status))
            }
            Packet::EncryptionRequest(_) => None,
            p => Some(p),
        }
    }
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
    async fn new(running: CancellationToken) -> Result<Self, Self::Error> {
        let config = Config::instance();
        let bind_address = format!("0.0.0.0:{}", config.proxy.port);

        // Only allow one client to join at a time.
        let connections = DownstreamConnectionManager::new(Some(1));
        let listener = connections
            .spawn_listener(bind_address, running.child_token())
            .await
            .map_err(Error::Network)?;

        let upstream_address = format!(
            "{}:{}",
            config.proxy.upstream_host, config.proxy.upstream_port
        );
        info!("Upstream server: {}", upstream_address);
        let upstream = Proxy::connect_upstream(&upstream_address).await?;

        Ok(Proxy {
            running,
            connections,
            listener,
            upstream,
            upstream_address,
        })
    }
    async fn update(&mut self) -> Result<(), Self::Error> {
        let _ = self.connections.update().await.map_err(Error::Network)?;

        let Some(client) = self.connections.clients_mut().take(1).next() else {
            return Ok(());
        };

        let mut client_error = false;
        let server_error = false;

        // At the same time, try to read packets from the server and client.
        // Forward the packet onto the other.
        tokio::select! {
            packet = client.read_packet() => {
                if let Some(packet) = packet {
                    match packet {
                        Ok(packet) => {
                            let next_state = packet.state_change();
                            if let Some(packet) = Proxy::rewrite_packet(packet) {
                                self.upstream.send_packet(packet).await.map_err(Error::Network)?;
                            }
                            if let Some(next_state) = next_state {
                                *self.upstream.client_state_mut() = next_state;
                            }
                        }
                        Err(e) => {
                            client_error = true;
                            match e {
                                NetworkError::Parsing => {
                                    trace!("Got invalid data from client (id {})", client.id());
                                    return Err(Error::Network(NetworkError::Parsing));
                                }
                                NetworkError::Io(e) => {
                                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                                        trace!("Client (id {}) disconnected", client.id());
                                    } else {
                                        trace!("Got IO error from client (id {}): {}", client.id(), e);
                                        return Err(Error::Io(e));
                                    }
                                }
                                e => return Err(Error::Network(e)),
                            };
                        }
                    }
                }
            }
            packet = self.upstream.read_packet() => {
                if let Some(packet) = packet {
                    match packet {
                        Ok(packet) => {
                            let next_state = packet.state_change();
                            if let Some(packet) = Proxy::rewrite_packet(packet) {
                                client.send_packet(packet).await.map_err(Error::Network)?;
                            }
                            if let Some(next_state) = next_state {
                                *client.inner_state_mut() = next_state;
                            }
                        }
                        Err(e) => {
                            // server_error = true;
                            return match e {
                                NetworkError::Parsing => {
                                    trace!("Got invalid data from upstream");
                                    Err(Error::Network(NetworkError::Parsing))
                                }
                                NetworkError::Io(e) => {
                                    trace!("Got IO error from upstream");
                                    Err(Error::Io(e))
                                }
                                e => Err(Error::Network(e)),
                            };
                        }
                    }
                }
            }
        }

        if client_error {
            let id = client.id();
            // Drop the &mut Connection
            let _ = client;
            let _ = self
                .connections
                .disconnect(
                    id,
                    Some(serde_json::json!({ "text": "Received malformed data." })),
                )
                .await;
        }
        if self.upstream.client_state() == ClientState::Disconnected || server_error {
            // Start a new connection with the upstream server.
            self.upstream = Proxy::connect_upstream(&self.upstream_address).await?;
        }

        Ok(())
    }
    async fn shutdown(self) -> Result<(), Self::Error> {
        // Ensure any child tasks have been shut down.
        self.running.cancel();

        let _ = self.listener.await.map_err(Error::Task)?;
        let _ = self
            .connections
            .shutdown(None)
            .await
            .map_err(Error::Network)?;

        Ok(())
    }
}
