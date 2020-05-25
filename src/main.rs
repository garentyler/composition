#![allow(non_snake_case)]

pub static DEBUG_LOGGING: bool = true;
pub static SERVER_VERSION: &str = "1.15.2";
pub static SERVER_PROTOCOL_VERSION: usize = 578;
pub static SERVER_MOTD: &str = "ligma balls";

// mod net;
// mod mctypes;

fn main() -> std::io::Result<()> {
    // Listener loops forever.
    // net::start_listener().expect("could not start listener");
    // println!("Stopping server");
    use std::{io::prelude::*, net::TcpStream};
    let mut stream = TcpStream::connect("206.189.67.44:25565")?;
}
