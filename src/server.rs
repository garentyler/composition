// server.rs
// author: Garen Tyler
// description:
//   Contains the server logic.

use crate::network::NetworkServer;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

pub struct Server {
    pub config: ServerConfig,
    pub receiver: Receiver<ServerMessage>,
    pub network: NetworkServer,
}
impl Server {
    pub fn update(&mut self) {
        // Do a tick.
        while let Ok(message) = self.receiver.try_recv() {
            debug!("Server received message: {:?}", message);
            self.handle_message(message);
        }
        self.network.update();
    }
    fn handle_message(&self, message: ServerMessage) {}
    pub fn shutdown(&self) {
        unimplemented!();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub protocol_version: u16,
    pub max_players: u32,
    pub motd: String,
    pub favicon: Option<String>,
}
impl ServerConfig {
    pub fn default() -> ServerConfig {
        ServerConfig {
            port: 25565,
            protocol_version: 578,
            max_players: 250,
            motd: "Hello world!".to_owned(),
            favicon: None,
        }
    }
    pub fn from_file(filename: &str) -> ServerConfig {
        use std::fs::File;
        use std::io::prelude::*;
        let a = || -> std::io::Result<ServerConfig> {
            let mut file = File::open(filename)?;
            let mut configStr = String::new();
            file.read_to_string(&mut configStr)?;
            Ok(toml::from_str(&configStr)?)
        };
        if let Ok(c) = a() {
            c
        } else {
            warn!(
                "Could not load config from {}, using default config.",
                filename
            );
            ServerConfig::default()
        }
    }
}

#[derive(Debug)]
pub enum BroadcastMessage {
    Shutdown,
}
#[derive(Debug)]
pub enum ServerMessage {}
