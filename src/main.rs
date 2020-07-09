// main.rs
// authors: Garen Tyler, Danton Hou
// description:
//   Main Game loop, config handler.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate lazy_static;
extern crate serde;
pub mod logger;
pub mod mctypes;
pub mod net;
pub mod protocol;

use serde::{Deserialize, Serialize};

lazy_static! {
    static ref log: logger::Logger = logger::new("log.txt");
    static ref config: Config = { Config::from_file("composition.toml") };
}

fn main() {
    // Start the network thread.
    std::thread::spawn(|| {
        log.info("Network thread started");
        net::start_listening();
    });
    // Loop the main thread for now.
    loop {}
}

// Not in it's own config module because of name conflicts.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub protocol_version: u16,
    pub max_players: u32,
}
impl Config {
    pub fn default() -> Config {
        Config {
            port: 25565,
            protocol_version: 578,
            max_players: 250,
        }
    }
    pub fn from_file(filename: &str) -> Config {
        use std::fs::File;
        use std::io::prelude::*;
        let a = || -> std::io::Result<Config> {
            let mut file = File::open(filename)?;
            let mut configStr = String::new();
            file.read_to_string(&mut configStr)?;
            Ok(toml::from_str(&configStr)?)
        };
        if let Ok(c) = a() {
            c
        } else {
            log.warn(&format!(
                "Could not load config from {}, using default config.",
                filename
            ));
            Config::default()
        }
    }
}
