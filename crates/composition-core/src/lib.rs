#![allow(dead_code)]

/// Server configuration and cli options.
pub mod config;
/// When managing the server encounters errors.
pub(crate) mod error;
/// Network operations.
pub(crate) mod net;
/// The core server implementation.
pub(crate) mod server;
/// World and block management.
pub(crate) mod world;

use crate::config::Config;
use once_cell::sync::OnceCell;
use std::time::Instant;

/// A globally accessible instant of the server's start time.
///
/// This should be set immediately on startup.
pub static START_TIME: OnceCell<Instant> = OnceCell::new();

/// Start the server.
#[tracing::instrument]
pub async fn start_server() -> server::Server {
    let bind_address = format!("0.0.0.0:{}", Config::instance().port);
    match server::Server::new(&bind_address).await {
        Ok(server) => server,
        Err(error::Error::Bind) => {
            tracing::error!("Could not bind to given address: {}", bind_address);
            std::process::exit(1);
        }
        Err(_) => unreachable!(),
    }
}
