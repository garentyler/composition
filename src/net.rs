// net.rs
// author: Garen Tyler
// description:
//   The module with everything to do with networkng.

use crate::{config, log};
use std::net::{TcpListener, TcpStream};

use crate::mctypes::*;
use crate::protocol::*;

pub fn start_listening() {
    let server_address: &str = &format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(server_address);
    if listener.is_err() {
        log.error("Could not start listener");
    } else {
        log.important(&format!("Started server on {}", server_address));
        for stream in listener.unwrap().incoming() {
            if stream.is_err() {
                log.error("Could not connect to client");
            } else {
                std::thread::spawn(move || {
                    if let Err(e) = handle_client(stream.unwrap()) {
                        log.error(&format!("Error when handling client: {}", e));
                    }
                });
            }
        }
    }
}
fn handle_client(t: TcpStream) -> std::io::Result<()> {
    log.info("Got a client!");
    let mut gc = GameConnection {
        stream: t,
        state: GameState::Handshake,
        protocol_version: 0,
    };

    'main: loop {
        match gc.state {
            GameState::Handshake => {
                // Read the handshake packet.
                let (_packet_len, packet_id) = read_packet_header(&mut gc.stream)?;
                let handshake = Handshake::read(&mut gc.stream)?;
                log.info(&format!("{:?}", handshake));
                gc.state = if handshake.protocol_version.value != config.protocol_version as i32 {
                    GameState::Closed
                } else {
                    match handshake.next_state.value {
                        1 => GameState::Status,
                        2 => GameState::Login,
                        _ => GameState::Closed,
                    }
                };
                log.info(&format!("Next state: {:?}", gc.state));
                gc.protocol_version = handshake.protocol_version.value as u16;
            }
            GameState::Status => {
                // Read the request packet.
                let (_request_packet_len, _request_packet_id) = read_packet_header(&mut gc.stream)?;
                // Send the response packet.
                let response = MCString::from(
                    r#"{
    "version": {
        "name": "1.15.2",
        "protocol": 578
    },
    "players": {
        "max": 420,
        "online": 69,
        "sample": [
            {
                "name": "thinkofdeath",
                "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
            }
        ]
    },
    "description": {
        "text": "ligma balls lol"
    },
    "favicon": "data:image/png;base64,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"
}"#,
                );
                let packet_id = MCVarInt::from(0x00);
                let packet_len = MCVarInt::from(
                    packet_id.to_bytes().len() as i32 + response.to_bytes().len() as i32,
                );
                for b in packet_len.to_bytes() {
                    write_byte(&mut gc.stream, b)?;
                }
                for b in packet_id.to_bytes() {
                    write_byte(&mut gc.stream, b)?;
                }
                for b in response.to_bytes() {
                    write_byte(&mut gc.stream, b)?;
                }
                // Read the ping packet.
                let (_ping_packet_len, _ping_packet_id) = read_packet_header(&mut gc.stream)?;
                let num = MCLong::from_stream(&mut gc.stream)?;
                log.info(&format!("Ping number: {:?}", num));
                // Send the pong packet.
                let packet_id = MCVarInt::from(0x01);
                let packet_len = MCVarInt::from(packet_id.to_bytes().len() as i32 + 8i32);
                for b in packet_len.to_bytes() {
                    write_byte(&mut gc.stream, b)?;
                }
                for b in packet_id.to_bytes() {
                    write_byte(&mut gc.stream, b)?;
                }
                for b in num.to_bytes() {
                    write_byte(&mut gc.stream, b)?;
                }
                gc.state = GameState::Closed;
            }
            GameState::Login => {
                // Read the login start packet.
                let (_packet_len, packet_id) = read_packet_header(&mut gc.stream)?;
                let login = LoginStart::read(&mut gc.stream)?;
                log.info(&format!("{:?}", login));
            }
            GameState::Play => {}
            GameState::Closed => {
                log.info(&format!(
                    "Client at {} closed connection",
                    gc.stream.peer_addr().unwrap()
                ));
                break 'main;
            }
        }
    }

    Ok(())
}

// pub struct MCPacket {
//     pub id: MCVarInt,
//     pub data: Vec<u8>,
// }
// #[allow(dead_code)]
// impl MCPacket {
//     pub fn read_header(t: &mut TcpStream) -> std::io::Result<(MCVarInt, MCVarInt)> {
//         let length = MCVarInt::from_stream(t)?;
//         let id = MCVarInt::from_stream(t)?;
//         Ok((length, id))
//     }
//     pub fn new(id: u8) -> MCPacket {
//         MCPacket {
//             id: MCVarInt::new(id as i32),
//             data: Vec::new(),
//         }
//     }
//     pub fn write(&mut self, v: Vec<u8>) {
//         for b in v {
//             self.data.push(b);
//         }
//     }
//     pub fn to_bytes(&self) -> Vec<u8> {
//         let mut bytes = Vec::new();
//         for b in MCVarInt::new((self.id.to_bytes().len() + self.data.len()) as i32).to_bytes() {
//             bytes.push(b);
//         }
//         for b in self.id.to_bytes() {
//             bytes.push(b);
//         }
//         for b in &self.data {
//             bytes.push(*b);
//         }
//         bytes
//     }
// }
#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum GameState {
    Handshake,
    Status,
    Login,
    Play,
    Closed,
}
#[allow(dead_code)]
pub struct GameConnection {
    pub stream: TcpStream,
    pub state: GameState,
    pub protocol_version: u16,
}
