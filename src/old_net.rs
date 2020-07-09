pub static SERVER_LISTENER_ADDRESS: &str = "127.0.0.1:25565";
pub static SOCKET_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);
extern crate serde;
extern crate serde_json;
use crate::mctypes::*;
use serde::Serialize;
use serde_json::json;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct MCPacket {
    pub id: MCVarInt,
    pub data: Vec<u8>,
}
#[allow(dead_code)]
impl MCPacket {
    pub fn read_header(t: &mut TcpStream) -> std::io::Result<(MCVarInt, MCVarInt)> {
        let length = MCVarInt::from_stream(t)?;
        let id = MCVarInt::from_stream(t)?;
        Ok((length, id))
    }
    pub fn new(id: u8) -> MCPacket {
        MCPacket {
            id: MCVarInt::new(id as i32),
            data: Vec::new(),
        }
    }
    pub fn write(&mut self, v: Vec<u8>) {
        for b in v {
            self.data.push(b);
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for b in MCVarInt::new((self.id.to_bytes().len() + self.data.len()) as i32).to_bytes() {
            bytes.push(b);
        }
        for b in self.id.to_bytes() {
            bytes.push(b);
        }
        for b in &self.data {
            bytes.push(*b);
        }
        bytes
    }
}
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
}

pub fn start_listener() -> std::io::Result<()> {
    if crate::DEBUG_LOGGING {
        println!("Started listener at {}", SERVER_LISTENER_ADDRESS);
    }
    let listener = TcpListener::bind(SERVER_LISTENER_ADDRESS)?;
    // Spawn a new thread for each connection.
    for stream in listener.incoming() {
        let stream = stream?;
        thread::Builder::new()
            .name(format!("GameConnection {}", stream.peer_addr().unwrap()))
            .spawn(move || -> std::io::Result<()> {
                if crate::DEBUG_LOGGING {
                    println!("Client connected at {}", stream.peer_addr().unwrap());
                }
                stream
                    .set_read_timeout(Some(SOCKET_TIMEOUT))
                    .expect("set_read_timeout call failed");
                stream
                    .set_write_timeout(Some(SOCKET_TIMEOUT))
                    .expect("set_write_timeout call failed");
                handle_client(GameConnection {
                    stream: stream,
                    state: GameState::Handshake,
                })?;
                Ok(())
            })?;
    }
    Ok(())
}
pub fn handle_client(mut gc: GameConnection) -> std::io::Result<()> {
    loop {
        let (packet_length, packet_id) = MCPacket::read_header(&mut gc.stream)?;
        if crate::DEBUG_LOGGING {
            println!(
                "Packet Length: {}, Packet ID: {}",
                packet_length.value, packet_id.value
            );
        }
        match gc.state {
            GameState::Handshake => match packet_id.value {
                0x00 => {
                    handshake(&mut gc)?;
                }
                _ => {
                    if crate::DEBUG_LOGGING {
                        println!("Unknown packet id {} in Handshake", packet_id);
                    }
                }
            },
            GameState::Login => match packet_id.value {
                0x00 => {
                    login(&mut gc)?;
                }
                _ => {
                    if crate::DEBUG_LOGGING {
                        println!("Unknown packet id {} in Login", packet_id);
                    }
                }
            },
            GameState::Status => {
                match packet_id.value {
                    0x00 => {
                        // Send a response packet.
                        let mut packet = MCPacket::new(0x00);
                        let json_response = json!({
                            "version": {
                                "name": crate::SERVER_VERSION,
                                "protocol": crate::SERVER_PROTOCOL_VERSION
                            },
                            "players": {
                                "max": 100,
                                "online": 5,
                                "sample": [
                                    {
                                        "name": "thinkofdeath",
                                        "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                                    }
                                ]
                            },
                            "description": {
                                "text": crate::SERVER_MOTD
                            }
                            // No favicon for now.
                            // "favicon": "data:image/png;base64,<data>"
                        })
                        .to_string();
                        packet.write(MCVarInt::new(json_response.len() as i32).to_bytes());
                        packet.write(MCString::from(json_response.clone()).to_bytes());
                        gc.stream.write(&packet.to_bytes())?;
                        println!("=== SENT SERVER RESPONSE ===\n{}", json_response);
                    }
                    _ => {
                        if crate::DEBUG_LOGGING {
                            println!("Unknown packet id {} in Status", packet_id);
                        }
                    }
                }
            }
            _ => {
                if crate::DEBUG_LOGGING {
                    println!("Unknown gamestate {:?}", gc.state);
                }
            }
        }
    }
}
pub fn handshake(gc: &mut GameConnection) -> std::io::Result<()> {
    // C->S Handshake
    let protocol_version = MCVarInt::from_stream(&mut gc.stream)?;
    let server_address = MCString::from_stream(&mut gc.stream)?;
    let server_port = MCUnsignedShort::from_stream(&mut gc.stream)?;
    let next_state = match MCVarInt::from_stream(&mut gc.stream)?.value {
        1 => GameState::Status,
        2 => GameState::Login,
        _ => {
            if crate::DEBUG_LOGGING {
                println!("Unknown next_state in handshake");
            }
            GameState::Handshake
        }
    };
    if crate::DEBUG_LOGGING {
        println!(
            "Handshake: Protocol Version: {}, Server Address: {}:{}, Next State: {:?}",
            protocol_version.value, server_address.value, server_port.value, next_state
        );
    }
    gc.state = next_state;
    Ok(())
}
pub fn login(gc: &mut GameConnection) -> std::io::Result<()> {
    // C->S Login Start
    let player_username = MCString::from_stream(&mut gc.stream)?;
    if crate::DEBUG_LOGGING {
        println!("Login: Player Username: {}", player_username);
    }
    // S->C Encryption Request
    // C->S Encryption Response
    // S->C Set Compression
    // S->C Login Success
    let mut login_success = MCPacket::new(0x02);
    login_success.write(MCString::from("00000000-0000-0000-0000-000000000000").to_bytes()); // UUID
    login_success.write(player_username.to_bytes());
    gc.stream.write(&login_success.to_bytes())?;
    // Move to Play state
    gc.state = GameState::Play;
    play(gc)?;
    Ok(())
}
pub fn play(gc: &mut GameConnection) -> std::io::Result<()> {
    Ok(())
}
