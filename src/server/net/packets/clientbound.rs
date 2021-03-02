use super::Packet;
use crate::mctypes::*;
use std::convert::{Into, TryFrom};
use std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct StatusResponse {
    pub json_response: MCString,
}
impl Into<Vec<u8>> for StatusResponse {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x00).into(); // 0x00 Status Response.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.json_response));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for StatusResponse {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
impl Packet for StatusResponse {
    fn new() -> Self {
        StatusResponse {
            json_response: MCString::from(""),
        }
    }
    fn read(t: &mut TcpStream) -> std::io::Result<Self> {
        let mut statusresponse = StatusResponse::new();
        statusresponse.json_response = MCString::read(t)?;
        Ok(statusresponse)
    }
    fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StatusPong {
    pub payload: MCLong,
}
impl Into<Vec<u8>> for StatusPong {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x01).into(); // 0x01 Status Pong.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.payload));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for StatusPong {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
impl Packet for StatusPong {
    fn new() -> Self {
        StatusPong { payload: 0.into() }
    }
    fn read(t: &mut TcpStream) -> std::io::Result<Self> {
        let mut statuspong = StatusPong::new();
        statuspong.payload = MCLong::read(t)?;
        Ok(statuspong)
    }
    fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LoginSuccess {
    pub uuid: MCString,
    pub username: MCString,
}
impl Into<Vec<u8>> for LoginSuccess {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x02).into(); // 0x02 Login Success.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.uuid));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.username));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for LoginSuccess {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
impl Packet for LoginSuccess {
    fn new() -> Self {
        LoginSuccess {
            uuid: MCString::from(""),
            username: MCString::from(""),
        }
    }
    fn read(t: &mut TcpStream) -> std::io::Result<Self> {
        let mut loginsuccess = LoginSuccess::new();
        loginsuccess.uuid = MCString::read(t)?;
        loginsuccess.username = MCString::read(t)?;
        Ok(loginsuccess)
    }
    fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LoginDisconnect {
    pub reason: MCChat,
}
impl Into<Vec<u8>> for LoginDisconnect {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x00).into(); // 0x00 Login Disconnect.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.reason));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for LoginDisconnect {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
impl Packet for LoginDisconnect {
    fn new() -> Self {
        LoginDisconnect {
            reason: MCChat {
                text: MCString::from(""),
            },
        }
    }
    fn read(t: &mut TcpStream) -> std::io::Result<Self> {
        let mut logindisconnect = LoginDisconnect::new();
        logindisconnect.reason = MCChat {
            text: MCString::read(t)?,
        };
        Ok(logindisconnect)
    }
    fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b)?;
        }
        Ok(())
    }
}
