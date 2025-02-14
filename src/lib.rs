/// Server configuration and cli options.
pub mod config;
/// Network operations.
pub(crate) mod net;
/// The Minecraft protocol implemented in a network-agnostic way.
pub mod protocol;
/// A proxy server.
#[cfg(feature = "proxy")]
pub(crate) mod proxy;
/// The core server implementation.
#[cfg(feature = "server")]
pub(crate) mod server;
/// A Minecraft world generator implementation that allows for custom worlds.
#[cfg(feature = "world")]
pub(crate) mod world;

use config::Subcommand;
use once_cell::sync::OnceCell;
use std::time::Instant;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

pub const PROTOCOL_VERSION: i32 = 767;
pub const GAME_VERSION: &str = "1.21.1";

/// A globally accessible instant of Composition's start time.
///
/// This should be set immediately on startup.
pub static START_TIME: OnceCell<Instant> = OnceCell::new();

pub async fn run(command: Subcommand, running: CancellationToken) {
    match command {
        #[cfg(feature = "server")]
        Subcommand::Server => server::Server::run(running).await,
        #[cfg(feature = "proxy")]
        Subcommand::Proxy => proxy::Proxy::run(running).await,
        Subcommand::None => unreachable!(),
    }
}

#[async_trait::async_trait]
pub(crate) trait App: Sized {
    type Error: std::fmt::Debug;

    fn startup_message() -> String;
    async fn new(running: CancellationToken) -> Result<Self, Self::Error>;
    async fn update(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(self) -> Result<(), Self::Error>;

    async fn run(running: CancellationToken) {
        info!("{}", Self::startup_message());
        let mut app = Self::new(running.clone()).await.expect("app to start");
        info!(
            "Done! Start took {:?}",
            crate::START_TIME.get().unwrap().elapsed()
        );

        // The main loop.
        loop {
            tokio::select! {
                _ = running.cancelled() => {
                    break;
                }
                r = app.update() => {
                    match r {
                        Ok(_) => {},
                        Err(e) => {
                            error!("{:?}", e);
                            break;
                        }
                    }
                }
            }
        }

        // Run shutdown tasks.
        match tokio::time::timeout(std::time::Duration::from_secs(10), app.shutdown()).await {
            Ok(_) => std::process::exit(0),
            Err(_) => std::process::exit(1),
        }
    }
}
