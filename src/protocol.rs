// protocol.rs
// author: Garen Tyler
// description:
//   This module contains all the packet structs.
//   Not all of them are implemented, and new ones will be added as necessary.

use crate::mctypes::*;
use std::net::TcpStream;

pub fn read_packet_header(t: &mut TcpStream) -> std::io::Result<(MCVarInt, MCVarInt)> {
    let length = MCVarInt::from_stream(t)?;
    let id = MCVarInt::from_stream(t)?;
    Ok((length, id))
}

#[derive(Debug, Clone)]
pub struct Handshake {
    pub protocol_version: MCVarInt,
    pub server_address: MCString,
    pub server_port: MCUnsignedShort,
    pub next_state: MCVarInt,
}
impl Handshake {
    pub fn new(
        protocol_version: MCVarInt,
        server_address: MCString,
        server_port: MCUnsignedShort,
        next_state: MCVarInt,
    ) -> Handshake {
        Handshake {
            protocol_version,
            server_address,
            server_port,
            next_state,
        }
    }
    pub fn read(t: &mut TcpStream) -> std::io::Result<Handshake> {
        let protocol_version = MCVarInt::from_stream(t)?;
        let server_address = MCString::from_stream(t)?;
        let server_port = MCUnsignedShort::from_stream(t)?;
        let next_state = MCVarInt::from_stream(t)?;
        Ok(Handshake::new(
            protocol_version,
            server_address,
            server_port,
            next_state,
        ))
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for b in self.protocol_version.to_bytes() {
            bytes.push(b);
        }
        for b in self.server_address.to_bytes() {
            bytes.push(b);
        }
        for b in self.server_port.to_bytes() {
            bytes.push(b);
        }
        for b in self.next_state.to_bytes() {
            bytes.push(b);
        }
        bytes
    }
}

#[derive(Debug, Clone)]
pub struct LoginStart {
    pub username: MCString,
}
impl LoginStart {
    pub fn new(username: MCString) -> LoginStart {
        LoginStart { username }
    }
    pub fn read(t: &mut TcpStream) -> std::io::Result<LoginStart> {
        Ok(LoginStart::new(MCString::from_stream(t)?))
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.username.to_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct LoginSuccess {
    pub uuid: MCString,
    pub username: MCString,
}
impl LoginSuccess {
    pub fn new(uuid: MCString, username: MCString) -> LoginSuccess {
        LoginSuccess { uuid, username }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for b in self.uuid.to_bytes() {
            bytes.push(b);
        }
        for b in self.username.to_bytes() {
            bytes.push(b);
        }
        bytes
    }
}
