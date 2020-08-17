// main.rs
// authors: Garen Tyler, Danton Hou
// description:
//   Initializes the server, main server loop.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

extern crate backtrace;
extern crate fern;
extern crate log;
extern crate serde;

pub mod network;
pub mod server;

use backtrace::Backtrace;
use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, error, info, warn};
use network::NetworkServer;
use serde::{Deserialize, Serialize};
use server::{Server, ServerConfig};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

fn main() {
    // Setup logging.
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{date} [{level}] - {message}",
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                // target = record.target(),
                level = record.level(),
                message = message,
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .unwrap();

    std::panic::set_hook(Box::new(|panic_info| {
        let backtrace = Backtrace::new();
        error!("{}\n{:?}", panic_info.to_string(), backtrace);
    }));

    info!("Starting server...");
    let start_time = Instant::now();

    let config = ServerConfig::from_file("composition.toml");
    let port = config.port;

    // Create the message channels.
    let (tx, rx) = mpsc::channel();

    // Create the server.
    let mut server = Server {
        config,
        receiver: rx,
        network: NetworkServer::new(port),
    };

    info!("Done! Start took {:?}", start_time.elapsed());

    // The main server loop.
    loop {
        server.update(); // Do the tick.
        std::thread::sleep(Duration::from_millis(50));
    }
}
