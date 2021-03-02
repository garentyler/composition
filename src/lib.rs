/// Data types for every entity in the game.
pub mod entity;
/// Implementations of the data types needed for the Minecraft protocol.
pub mod mctypes;
/// The logic for the server.
pub mod server;
/// The data types for blocks, chunks, dimensions, and world files.
pub mod world;

pub use mctypes::*;

/// Set up logging, read the config file, etc.
pub fn init() {
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
}

/// Start the server.
pub async fn start_server() -> server::GameServer {
    // Start the network.
    let network = server::net::NetworkServer::new("0.0.0.0:25565");
    let server = server::GameServer { network: network };
    server
}
