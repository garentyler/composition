#![allow(unused_imports)]
#![allow(non_snake_case)]

pub mod mctypes;
pub mod net;
pub mod server;
extern crate chrono;
extern crate fern;
extern crate log;

use log::{debug, error, info, warn};
use net::NetworkServer;
use server::GameServer;
use std::time::{Duration, Instant};

pub fn main() {
    let start_time = Instant::now();

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
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .unwrap();
    info!("Starting server...");

    // Start the network.
    let network = NetworkServer::new("0.0.0.0:25565");
    let mut server = GameServer { network: network };
    info!("Done! Start took {:?}", start_time.elapsed());

    // The main server loop.
    loop {
        server.update();
        std::thread::sleep(Duration::from_millis(2));
    }
}
