use crate::mctypes::*;
use crate::CONFIG;
use std::convert::{Into, TryFrom};
use tokio::net::TcpStream;

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
impl StatusResponse {
    pub fn new() -> Self {
        StatusResponse {
            json_response: MCString::from(""),
        }
    }
    pub async fn read(t: &'_ mut TcpStream) -> tokio::io::Result<Self> {
        let mut statusresponse = StatusResponse::new();
        statusresponse.json_response = MCString::read(t).await?;
        Ok(statusresponse)
    }
    pub async fn write(&self, t: &'_ mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
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
impl StatusPong {
    pub fn new() -> Self {
        StatusPong { payload: 0.into() }
    }
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut statuspong = StatusPong::new();
        statuspong.payload = MCLong::read(t).await?;
        Ok(statuspong)
    }
    pub async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
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
impl LoginSuccess {
    pub fn new() -> Self {
        LoginSuccess {
            uuid: MCString::from(""),
            username: MCString::from(""),
        }
    }
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut loginsuccess = LoginSuccess::new();
        loginsuccess.uuid = MCString::read(t).await?;
        loginsuccess.username = MCString::read(t).await?;
        Ok(loginsuccess)
    }
    pub async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
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
impl LoginDisconnect {
    pub fn new() -> Self {
        LoginDisconnect {
            reason: MCChat {
                text: MCString::from(""),
            },
        }
    }
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut logindisconnect = LoginDisconnect::new();
        logindisconnect.reason = MCChat {
            text: MCString::read(t).await?,
        };
        Ok(logindisconnect)
    }
    pub async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct JoinGame {
    entity_id: MCInt, // The player's Entity ID (EID)
    gamemode: MCUnsignedByte, // 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. Bit 3 (0x8) is the hardcore flag.
    dimension: MCByte, // -1: Nether, 0: Overworld, 1: End
    difficulty: MCUnsignedByte, // 0: Peaceful, 1: Easy, 2: Normal, 3: Hard
    max_players: MCUnsignedByte, // Used by the client to draw the player list
    level_type: MCString, // default, flat, largeBiomes, amplified, default_1_1
    reduced_debug_info: MCBoolean, // If true, a Notchian client shows reduced information on the debug screen.
}
impl Into<Vec<u8>> for JoinGame {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x01).into(); // 0x01 Join Game.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.entity_id));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.gamemode));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.dimension));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.difficulty));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.max_players));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.level_type));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.reduced_debug_info));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for JoinGame {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
impl JoinGame {
    pub fn new() -> Self {
        JoinGame {
            entity_id: 0.into(),
            gamemode: 1.into(), // Default to creative mode.
            dimension: 0.into(), // Default to overworld.
            difficulty: 2.into(),
            max_players: (CONFIG.max_players as u8).into(),
            level_type: "default".into(), // Use the default world type.
            reduced_debug_info: false.into(), // The debug info should be useful.
        }
    }
    pub async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut joingame = JoinGame::new();
        joingame.entity_id = MCInt::read(t).await?;
        joingame.gamemode = MCUnsignedByte::read(t).await?;
        joingame.dimension = MCByte::read(t).await?;
        joingame.difficulty = MCUnsignedByte::read(t).await?;
        joingame.max_players = MCUnsignedByte::read(t).await?;
        joingame.level_type = MCString::read(t).await?;
        joingame.reduced_debug_info = MCBoolean::read(t).await?;
        Ok(joingame)
    }
    pub async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}
