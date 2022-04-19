#[macro_use]
extern crate lazy_static;

pub mod config;
pub mod net;
pub mod server;

use crate::prelude::*;
use std::sync::mpsc::{self, Receiver};

pub static PROTOCOL_VERSION: i32 = 757;
lazy_static! {
    pub static ref CONFIG: Config = Config::from_file("composition.toml");
    pub static ref FAVICON: std::io::Result<Vec<u8>> = {
        use std::{fs::File, io::prelude::*};
        let mut data = vec![];
        let mut file = File::open(CONFIG.favicon.clone())?;
        file.read_to_end(&mut data)?;
        Ok(data)
    };
    pub static ref START_TIME: std::time::Instant = std::time::Instant::now();
}

/// Set up logging, read the config file, etc.
pub fn init() -> Receiver<()> {
    // Load the START_TIME static - lazy_static lazy loads the value when first needed.
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
    pub use crate::{config::Config, CONFIG, FAVICON, PROTOCOL_VERSION, START_TIME};
    pub use log::*;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::json;
    pub use uuid::Uuid;
    pub type JSON = serde_json::Value;
    pub type NBT = fastnbt::Value;
    pub use std::collections::VecDeque;
    pub use substring::Substring;
    pub use tokio::io::{AsyncReadExt, AsyncWriteExt};
    #[derive(Clone, Debug, PartialEq)]
    pub enum ParseError {
        NotEnoughData,
        InvalidData,
        VarIntTooBig,
    }
    pub type ParseResult<T> = Result<(T, usize), ParseError>;
}
