/// Server configuration and cli options.
pub mod config;
/// The Minecraft protocol implemented in a network-agnostic way.
pub mod protocol;
/// The core server implementation.
pub(crate) mod server;
/// A Minecraft world generator implementation that allows for custom worlds.
pub mod world;

use config::Subcommand;
use once_cell::sync::OnceCell;
use std::time::Instant;

/// A globally accessible instant of the composition's start time.
///
/// This should be set immediately on startup.
pub static START_TIME: OnceCell<Instant> = OnceCell::new();

pub async fn run(command: Subcommand) {
    match command {
        Subcommand::Server => server::Server::run().await,
        Subcommand::None => unreachable!(),
    }
}
