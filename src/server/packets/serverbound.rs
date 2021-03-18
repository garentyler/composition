use crate::mctypes::*;
use std::convert::{Into, TryFrom};
use tokio::net::TcpStream;

/// Needed for every interaction with the server.
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
impl Handshake {
    pub fn new() -> Self {
        Handshake {
            protocol_version: 0.into(),
            server_address: "".into(),
            server_port: 0.into(),
            next_state: 0.into(),
        }
    }
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut handshake = Handshake::new();
        handshake.protocol_version = MCVarInt::read(t).await?;
        handshake.server_address = MCString::read(t).await?;
        handshake.server_port = MCUnsignedShort::read(t).await?;
        handshake.next_state = MCVarInt::read(t).await?;
        Ok(handshake)
    }
    pub async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StatusRequest {}
impl Into<Vec<u8>> for StatusRequest {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let temp: Vec<u8> = MCVarInt::from(0x00).into(); // 0x00 Status Request.
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
impl StatusRequest {
    pub fn new() -> Self {
        StatusRequest {}
    }
    pub async fn read(_t: &mut TcpStream) -> tokio::io::Result<Self> {
        let statusrequest = StatusRequest::new();
        Ok(statusrequest)
    }
    pub async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
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
impl StatusPing {
    pub fn new() -> Self {
        StatusPing { payload: 0.into() }
    }
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut statusping = StatusPing::new();
        statusping.payload = MCLong::read(t).await?;
        Ok(statusping)
    }
    pub async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
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
impl LoginStart {
    pub fn new() -> Self {
        LoginStart {
            player_name: "".into(),
        }
    }
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut loginstart = LoginStart::new();
        loginstart.player_name = MCString::read(t).await?;
        Ok(loginstart)
    }
    pub async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ClientSettings {
    pub locale: MCString,
    pub view_distance: MCByte,
    pub chat_mode: MCVarInt, // 0: enabled, 1: commands only, 2: hidden.
    pub chat_colors: MCBoolean,
    pub displayed_skin_parts: MCUnsignedByte, // Bit mask
                                              // Displayed skin parts flags:
                                              // Bit 0 (0x01): Cape enabled
                                              // Bit 1 (0x02): Jacket enabled
                                              // Bit 2 (0x04): Left Sleeve enabled
                                              // Bit 3 (0x08): Right Sleeve enabled
                                              // Bit 4 (0x10): Left Pants Leg enabled
                                              // Bit 5 (0x20): Right Pants Leg enabled
                                              // Bit 6 (0x40): Hat enabled
}
impl Into<Vec<u8>> for ClientSettings {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x15).into(); // 0x15 Client Settings.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.locale));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.view_distance));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.chat_mode));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.chat_colors));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.displayed_skin_parts));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for ClientSettings {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
impl ClientSettings {
    pub fn new() -> Self {
        ClientSettings {
            locale: "en_US".into(),
            view_distance: 8.into(), // 8 chunks.
            chat_mode: 0.into(),     // All chat enabled.
            chat_colors: true.into(),
            displayed_skin_parts: 0xff.into(), // Enable all parts.
        }
    }
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut clientsettings = ClientSettings::new();
        clientsettings.locale = MCString::read(t).await?;
        clientsettings.view_distance = MCByte::read(t).await?;
        clientsettings.chat_mode = MCVarInt::read(t).await?;
        clientsettings.chat_colors = MCBoolean::read(t).await?;
        clientsettings.displayed_skin_parts = MCUnsignedByte::read(t).await?;
        Ok(clientsettings)
    }
    pub async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KeepAlivePong {
    payload: MCVarInt,
}
impl Into<Vec<u8>> for KeepAlivePong {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x00).into(); // 0x00 Keep Alive.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.payload));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for KeepAlivePong {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
impl KeepAlivePong {
    pub fn new() -> Self {
        KeepAlivePong { payload: 0.into() }
    }
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut keepalive = KeepAlivePong::new();
        keepalive.payload = MCVarInt::read(t).await?;
        Ok(keepalive)
    }
    pub async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}
