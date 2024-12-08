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

pub const PROTOCOL_VERSION: i32 = 762;
pub const GAME_VERSION: &str = "1.19.4";

/// A globally accessible instant of the composition's start time.
///
/// This should be set immediately on startup.
pub static START_TIME: OnceCell<Instant> = OnceCell::new();

pub async fn run(command: Subcommand) {
    match command {
        #[cfg(feature = "server")]
        Subcommand::Server => server::Server::run().await,
        #[cfg(feature = "proxy")]
        Subcommand::Proxy => proxy::Proxy::run().await,
        Subcommand::None => unreachable!(),
    }
}
