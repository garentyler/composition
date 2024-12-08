pub mod config;

use crate::{config::Config, net::listener::NetworkListener};
use config::ProxyConfig;
use tokio::net::ToSocketAddrs;
use tokio_util::sync::CancellationToken;
use tracing::{info, trace};

#[derive(Debug)]
pub struct Proxy {
    _network_listener: NetworkListener,
}
impl Proxy {
    /// Start the proxy.
    #[tracing::instrument]
    pub async fn run() {
        let config = Config::instance();
        info!(
            "Starting {} on port {}",
            ProxyConfig::default().version,
            config.proxy.port
        );
        let (mut proxy, running) = Self::new(format!("0.0.0.0:{}", config.proxy.port)).await;
        info!(
            "Done! Start took {:?}",
            crate::START_TIME.get().unwrap().elapsed()
        );
        info!(
            "Upstream server: {}:{}",
            config.proxy.upstream_host, config.proxy.upstream_port
        );

        // Spawn the ctrl-c task.
        let r = running.clone();
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            info!("Ctrl-C received, shutting down");
            r.cancel();
        });

        // The main loop.
        loop {
            tokio::select! {
                _ = running.cancelled() => {
                    break;
                }
                _ = proxy.update() => {}
            }
        }

        match tokio::time::timeout(std::time::Duration::from_secs(10), proxy.shutdown()).await {
            Ok(_) => std::process::exit(0),
            Err(_) => std::process::exit(1),
        }
    }
    #[tracing::instrument]
    async fn new<A: 'static + ToSocketAddrs + Send + std::fmt::Debug>(
        bind_address: A,
    ) -> (Proxy, CancellationToken) {
        trace!("Proxy::new()");
        let running = CancellationToken::new();

        let network_listener = NetworkListener::new(bind_address, running.child_token(), None)
            .await
            .expect("listener to bind properly");

        let proxy = Proxy {
            _network_listener: network_listener,
        };

        (proxy, running)
    }
    #[tracing::instrument]
    async fn update(&mut self) -> Result<(), ()> {
        // TODO
        Ok(())
    }
    #[tracing::instrument]
    async fn shutdown(self) {
        trace!("Proxy.shutdown()");
        // TODO
    }
}
