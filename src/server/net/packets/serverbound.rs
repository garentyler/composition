use super::PacketCommon;
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
        let mut temp: Vec<u8> = MCVarInt::from(Self::id()).into(); // 0x00 Handshake.
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
#[async_trait::async_trait]
impl PacketCommon for Handshake {
    fn new() -> Self {
        Handshake {
            protocol_version: 0.into(),
            server_address: "".into(),
            server_port: 0.into(),
            next_state: 0.into(),
        }
    }
    fn id() -> u8 {
        0x00
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut handshake = Handshake::new();
        handshake.protocol_version = MCVarInt::read(t).await?;
        handshake.server_address = MCString::read(t).await?;
        handshake.server_port = MCUnsignedShort::read(t).await?;
        handshake.next_state = MCVarInt::read(t).await?;
        Ok(handshake)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
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
        let temp: Vec<u8> = MCVarInt::from(Self::id()).into(); // 0x00 Status Request.
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
#[async_trait::async_trait]
impl PacketCommon for StatusRequest {
    fn new() -> Self {
        StatusRequest {}
    }
    fn id() -> u8 {
        0x00
    }
    async fn read(_t: &mut TcpStream) -> tokio::io::Result<Self> {
        let statusrequest = StatusRequest::new();
        Ok(statusrequest)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
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
        let mut temp: Vec<u8> = MCVarInt::from(Self::id()).into(); // 0x01 Status Pong.
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
#[async_trait::async_trait]
impl PacketCommon for StatusPing {
    fn new() -> Self {
        StatusPing { payload: 0.into() }
    }
    fn id() -> u8 {
        0x01
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut statusping = StatusPing::new();
        statusping.payload = MCLong::read(t).await?;
        Ok(statusping)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
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
        let mut temp: Vec<u8> = MCVarInt::from(Self::id()).into(); // 0x00 Login Start.
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
#[async_trait::async_trait]
impl PacketCommon for LoginStart {
    fn new() -> Self {
        LoginStart {
            player_name: "".into(),
        }
    }
    fn id() -> u8 {
        0x00
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut loginstart = LoginStart::new();
        loginstart.player_name = MCString::read(t).await?;
        Ok(loginstart)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
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
        let mut temp: Vec<u8> = MCVarInt::from(Self::id()).into(); // 0x15 Client Settings.
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
#[async_trait::async_trait]
impl PacketCommon for ClientSettings {
    fn new() -> Self {
        ClientSettings {
            locale: "en_US".into(),
            view_distance: 8.into(), // 8 chunks.
            chat_mode: 0.into(),     // All chat enabled.
            chat_colors: true.into(),
            displayed_skin_parts: 0xff.into(), // Enable all parts.
        }
    }
    fn id() -> u8 {
        0x15
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut clientsettings = ClientSettings::new();
        clientsettings.locale = MCString::read(t).await?;
        clientsettings.view_distance = MCByte::read(t).await?;
        clientsettings.chat_mode = MCVarInt::read(t).await?;
        clientsettings.chat_colors = MCBoolean::read(t).await?;
        clientsettings.displayed_skin_parts = MCUnsignedByte::read(t).await?;
        Ok(clientsettings)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KeepAlivePong {
    pub payload: MCVarInt,
}
impl Into<Vec<u8>> for KeepAlivePong {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(Self::id()).into(); // 0x00 Keep Alive.
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
#[async_trait::async_trait]
impl PacketCommon for KeepAlivePong {
    fn new() -> Self {
        KeepAlivePong { payload: 0.into() }
    }
    fn id() -> u8 {
        0x00
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut keepalive = KeepAlivePong::new();
        keepalive.payload = MCVarInt::read(t).await?;
        Ok(keepalive)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServerboundChatMessage {
    pub text: MCString,
}
impl Into<Vec<u8>> for ServerboundChatMessage {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(Self::id()).into(); // 0x01 Serverbound Chat Message.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.text));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for ServerboundChatMessage {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for ServerboundChatMessage {
    fn new() -> Self {
        ServerboundChatMessage { text: "".into() }
    }
    fn id() -> u8 {
        0x01
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut serverboundchatmessage = ServerboundChatMessage::new();
        serverboundchatmessage.text = MCString::read(t).await?;
        Ok(serverboundchatmessage)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub on_ground: MCBoolean,
}
impl Into<Vec<u8>> for Player {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(Self::id()).into(); // 0x03 Player.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.on_ground));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for Player {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for Player {
    fn new() -> Self {
        Player {
            on_ground: false.into(),
        }
    }
    fn id() -> u8 {
        0x03
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut player = Player::new();
        player.on_ground = MCBoolean::read(t).await?;
        Ok(player)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PlayerPosition {
    pub x: MCDouble,
    pub y: MCDouble,
    pub z: MCDouble,
    pub on_ground: MCBoolean,
}
impl Into<Vec<u8>> for PlayerPosition {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(Self::id()).into(); // 0x04 Player Position.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.x));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.y));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.z));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.on_ground));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for PlayerPosition {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for PlayerPosition {
    fn new() -> Self {
        PlayerPosition {
            x: 0.0.into(),
            y: 0.0.into(),
            z: 0.0.into(),
            on_ground: false.into(),
        }
    }
    fn id() -> u8 {
        0x04
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut playerposition = PlayerPosition::new();
        playerposition.x = MCDouble::read(t).await?;
        playerposition.y = MCDouble::read(t).await?;
        playerposition.z = MCDouble::read(t).await?;
        playerposition.on_ground = MCBoolean::read(t).await?;
        Ok(playerposition)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PlayerLook {
    pub yaw: MCFloat,
    pub pitch: MCFloat,
    pub on_ground: MCBoolean,
}
impl Into<Vec<u8>> for PlayerLook {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(Self::id()).into(); // 0x05 Player Look.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.yaw));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.pitch));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.on_ground));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for PlayerLook {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for PlayerLook {
    fn new() -> Self {
        PlayerLook {
            yaw: 0.0.into(),
            pitch: 0.0.into(),
            on_ground: false.into(),
        }
    }
    fn id() -> u8 {
        0x05
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut playerlook = PlayerLook::new();
        playerlook.yaw = MCFloat::read(t).await?;
        playerlook.pitch = MCFloat::read(t).await?;
        playerlook.on_ground = MCBoolean::read(t).await?;
        Ok(playerlook)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServerboundPlayerPositionAndLook {
    pub x: MCDouble,
    pub y: MCDouble,
    pub z: MCDouble,
    pub yaw: MCFloat,
    pub pitch: MCFloat,
    pub on_ground: MCBoolean,
}
impl Into<Vec<u8>> for ServerboundPlayerPositionAndLook {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(Self::id()).into(); // 0x06 Serverbound Player Position And Look.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.x));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.y));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.z));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.yaw));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.pitch));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.on_ground));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for ServerboundPlayerPositionAndLook {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for ServerboundPlayerPositionAndLook {
    fn new() -> Self {
        ServerboundPlayerPositionAndLook {
            x: 0.0.into(),
            y: 0.0.into(),
            z: 0.0.into(),
            yaw: 0.0.into(),
            pitch: 0.0.into(),
            on_ground: false.into(),
        }
    }
    fn id() -> u8 {
        0x06
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut playerpositionandlook = ServerboundPlayerPositionAndLook::new();
        playerpositionandlook.x = MCDouble::read(t).await?;
        playerpositionandlook.y = MCDouble::read(t).await?;
        playerpositionandlook.z = MCDouble::read(t).await?;
        playerpositionandlook.yaw = MCFloat::read(t).await?;
        playerpositionandlook.pitch = MCFloat::read(t).await?;
        playerpositionandlook.on_ground = MCBoolean::read(t).await?;
        Ok(playerpositionandlook)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}
