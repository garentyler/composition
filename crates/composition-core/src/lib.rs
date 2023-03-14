#[macro_use]
extern crate lazy_static;

pub mod config;
pub mod net;
pub mod server;

use crate::prelude::*;
use std::sync::mpsc::{self, Receiver};

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
    pub static ref START_TIME: std::time::Instant = std::time::Instant::now();
}

/// Set up logging, read the config file, etc.
pub fn init() -> Receiver<()> {
    let _ = START_TIME.elapsed();
    // Set up fern logging.
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date}][{target}][{level}] {message}",
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = record.level(),
                message = message,
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .unwrap();
    log::set_max_level(CONFIG.log_level);
    // Set up the ctrl-c handler.
    let (ctrlc_tx, ctrlc_rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        ctrlc_tx.send(()).expect("Ctrl-C receiver disconnected");
    })
    .expect("Error setting Ctrl-C handler");
    ctrlc_rx
}

/// Start the server.
pub async fn start_server() -> server::Server {
    server::Server::new(format!("0.0.0.0:{}", CONFIG.port)).await
}

pub mod prelude {
    pub use crate::{config::Config, CONFIG, START_TIME};
    pub use log::*;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::json;
    pub use uuid::Uuid;
    pub type JSON = serde_json::Value;
    pub type NBT = quartz_nbt::NbtCompound;
    pub use std::collections::VecDeque;
    pub use std::io::{Read, Write};
    pub use substring::Substring;
    pub use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
    #[derive(Clone, Debug, PartialEq)]
    pub enum ParseError {
        NotEnoughData,
        InvalidData,
        VarIntTooBig,
    }
    pub type ParseResult<T> = Result<(T, usize), ParseError>;
}