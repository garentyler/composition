/// Server configuration and cli options.
pub mod config;
/// When managing the server encounters errors.
pub(crate) mod error;
/// Network operations.
pub(crate) mod net;
/// The Minecraft protocol implemented in a network-agnostic way.
pub mod protocol;
/// The core server implementation.
pub(crate) mod server;
/// A Minecraft world generator implementation that allows for custom worlds.
pub mod world;

use crate::config::Config;
use once_cell::sync::OnceCell;
use std::time::Instant;

/// A globally accessible instant of the server's start time.
///
/// This should be set immediately on startup.
pub static START_TIME: OnceCell<Instant> = OnceCell::new();

/// Start the server.
#[tracing::instrument]
pub async fn start_server() -> (server::Server, tokio_util::sync::CancellationToken) {
    server::Server::new(format!("0.0.0.0:{}", Config::instance().port)).await
}
