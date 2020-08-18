use super::Packet;
use crate::mctypes::*;
use std::convert::{Into, TryFrom};
use std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Handshake {
    pub protocol_version: MCVarInt,
    pub server_address: MCString,
    pub server_port: MCUnsignedShort,
    pub next_state: MCVarInt,
}
impl Into<Vec<u8>> for Handshake {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x00).into(); // 0x00 Handshake.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.protocol_version));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.server_address));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.server_port));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.next_state));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for Handshake {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
impl Packet for Handshake {
    fn new() -> Self {
        Handshake {
            protocol_version: 0.into(),
            server_address: "".into(),
            server_port: 0.into(),
            next_state: 0.into(),
        }
    }
    fn read(t: &mut TcpStream) -> std::io::Result<Self> {
        let mut handshake = Handshake::new();
        handshake.protocol_version = MCVarInt::read(t)?;
        handshake.server_address = MCString::read(t)?;
        handshake.server_port = MCUnsignedShort::read(t)?;
        handshake.next_state = MCVarInt::read(t)?;
        Ok(handshake)
    }
    fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StatusRequest {}
impl Into<Vec<u8>> for StatusRequest {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x00).into(); // 0x00 Status Request.
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for StatusRequest {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
impl Packet for StatusRequest {
    fn new() -> Self {
        StatusRequest {}
    }
    fn read(t: &mut TcpStream) -> std::io::Result<Self> {
        let mut statusrequest = StatusRequest::new();
        Ok(statusrequest)
    }
    fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StatusPing {
    pub payload: MCLong,
}
impl Into<Vec<u8>> for StatusPing {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x01).into(); // 0x01 Status Pong.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.payload));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for StatusPing {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
impl Packet for StatusPing {
    fn new() -> Self {
        StatusPing { payload: 0.into() }
    }
    fn read(t: &mut TcpStream) -> std::io::Result<Self> {
        let mut statusping = StatusPing::new();
        statusping.payload = MCLong::read(t)?;
        Ok(statusping)
    }
    fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LoginStart {
    pub player_name: MCString,
}
impl Into<Vec<u8>> for LoginStart {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x00).into(); // 0x00 Login Start.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.player_name));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for LoginStart {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
impl Packet for LoginStart {
    fn new() -> Self {
        LoginStart {
            player_name: "".into(),
        }
    }
    fn read(t: &mut TcpStream) -> std::io::Result<Self> {
        let mut loginstart = LoginStart::new();
        loginstart.player_name = MCString::read(t)?;
        Ok(loginstart)
    }
    fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b)?;
        }
        Ok(())
    }
}
