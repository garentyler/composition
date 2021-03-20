#[macro_use]
extern crate lazy_static;

/// Data types for every entity in the game.
pub mod entity;
/// Implementations of the data types needed for the Minecraft protocol.
pub mod mctypes;
/// The logic for the server.
pub mod server;
/// The data types for blocks, chunks, dimensions, and world files.
pub mod world;

use log::warn;
pub use mctypes::*;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, Receiver};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub max_players: usize,
    pub motd: String,
    pub favicon: String,
}

lazy_static! {
    static ref CONFIG: Config = {
        let config_from_file = || -> std::io::Result<Config> {
            use std::{fs::File, io::prelude::*};
            let mut data = String::new();
            let mut file = File::open("composition.toml")?;
            file.read_to_string(&mut data)?;
            if let Ok(c) = toml::from_str::<Config>(&data) {
                Ok(c)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Could not parse toml",
                ))
            }
        };
        if let Ok(c) = config_from_file() {
            c
        } else {
            warn!("Could not load config from file, using default");
            Config {
                port: 25565,
                max_players: 20,
                motd: "Hello world!".to_owned(),
                favicon: "server-icon.png".to_owned(),
            }
        }
    };
    static ref FAVICON: std::io::Result<Vec<u8>> = {
        use std::{fs::File, io::prelude::*};
        let mut data = vec![];
        let mut file = File::open(CONFIG.favicon.clone())?;
        file.read_to_end(&mut data)?;
        Ok(data)
    };
}

/// Set up logging, read the config file, etc.
pub fn init() -> Receiver<()> {
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
        .level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .unwrap();
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
    server::Server::new(format!("0.0.0.0:{}", CONFIG.port))
}
