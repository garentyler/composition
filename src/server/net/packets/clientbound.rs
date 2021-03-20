use super::PacketCommon;
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
#[async_trait::async_trait]
impl PacketCommon for StatusResponse {
    fn new() -> Self {
        StatusResponse {
            json_response: MCString::from(""),
        }
    }
    async fn read(t: &'_ mut TcpStream) -> tokio::io::Result<Self> {
        let mut statusresponse = StatusResponse::new();
        statusresponse.json_response = MCString::read(t).await?;
        Ok(statusresponse)
    }
    async fn write(&self, t: &'_ mut TcpStream) -> tokio::io::Result<()> {
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
#[async_trait::async_trait]
impl PacketCommon for StatusPong {
    fn new() -> Self {
        StatusPong { payload: 0.into() }
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut statuspong = StatusPong::new();
        statuspong.payload = MCLong::read(t).await?;
        Ok(statuspong)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
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
#[async_trait::async_trait]
impl PacketCommon for LoginSuccess {
    fn new() -> Self {
        LoginSuccess {
            uuid: MCString::from(""),
            username: MCString::from(""),
        }
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut loginsuccess = LoginSuccess::new();
        loginsuccess.uuid = MCString::read(t).await?;
        loginsuccess.username = MCString::read(t).await?;
        Ok(loginsuccess)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
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
#[async_trait::async_trait]
impl PacketCommon for LoginDisconnect {
    fn new() -> Self {
        LoginDisconnect {
            reason: MCChat {
                text: MCString::from(""),
            },
        }
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut logindisconnect = LoginDisconnect::new();
        logindisconnect.reason = MCChat {
            text: MCString::read(t).await?,
        };
        Ok(logindisconnect)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct JoinGame {
    pub entity_id: MCInt,              // The player's Entity ID (EID)
    pub gamemode: MCUnsignedByte, // 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. Bit 3 (0x8) is the hardcore flag.
    pub dimension: MCByte,        // -1: Nether, 0: Overworld, 1: End
    pub difficulty: MCUnsignedByte, // 0: Peaceful, 1: Easy, 2: Normal, 3: Hard
    pub max_players: MCUnsignedByte, // Used by the client to draw the player list
    pub level_type: MCString,     // default, flat, largeBiomes, amplified, default_1_1
    pub reduced_debug_info: MCBoolean, // If true, a Notchian client shows reduced information on the debug screen.
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
#[async_trait::async_trait]
impl PacketCommon for JoinGame {
    fn new() -> Self {
        JoinGame {
            entity_id: 0.into(),
            gamemode: 1.into(),  // Default to creative mode.
            dimension: 0.into(), // Default to overworld.
            difficulty: 2.into(),
            max_players: (CONFIG.max_players as u8).into(),
            level_type: "default".into(), // Use the default world type.
            reduced_debug_info: false.into(), // The debug info should be useful.
        }
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
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
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct HeldItemChange {
    pub selected_slot: MCByte,
}
impl Into<Vec<u8>> for HeldItemChange {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x09).into(); // 0x09 Held Item Change.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.selected_slot));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for HeldItemChange {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for HeldItemChange {
    fn new() -> Self {
        HeldItemChange {
            selected_slot: 0.into(),
        }
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut helditemchange = HeldItemChange::new();
        helditemchange.selected_slot = MCByte::read(t).await?;
        Ok(helditemchange)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct EntityStatus {
    pub entity_id: MCInt,
    pub entity_status: MCByte, // See table below.
                               // 1:  Sent when resetting a mob spawn minecart's timer / Rabbit jump animation
                               // 2:  Living Entity hurt
                               // 3:  Living Entity dead
                               // 4:  Iron Golem throwing up arms
                               // 6:  Wolf/Ocelot/Horse taming — Spawn “heart” particles
                               // 7:  Wolf/Ocelot/Horse tamed — Spawn “smoke” particles
                               // 8:  Wolf shaking water — Trigger the shaking animation
                               // 9:  (of self) Eating accepted by server
                               // 10: Sheep eating grass
                               // 10: Play TNT ignite sound
                               // 11: Iron Golem handing over a rose
                               // 12: Villager mating — Spawn “heart” particles
                               // 13: Spawn particles indicating that a villager is angry and seeking revenge
                               // 14: Spawn happy particles near a villager
                               // 15: Witch animation — Spawn “magic” particles
                               // 16: Play zombie converting into a villager sound
                               // 17: Firework exploding
                               // 18: Animal in love (ready to mate) — Spawn “heart” particles
                               // 19: Reset squid rotation
                               // 20: Spawn explosion particle — works for some living entities
                               // 21: Play guardian sound — works for only for guardians
                               // 22: Enables reduced debug for players
                               // 23: Disables reduced debug for players
}
impl Into<Vec<u8>> for EntityStatus {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x1a).into(); // 0x1a Entity Status.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.entity_id));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.entity_status));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for EntityStatus {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for EntityStatus {
    fn new() -> Self {
        EntityStatus {
            entity_id: 0.into(),
            entity_status: 0.into(),
        }
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut entitystatus = EntityStatus::new();
        entitystatus.entity_id = MCInt::read(t).await?;
        entitystatus.entity_status = MCByte::read(t).await?;
        Ok(entitystatus)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PlayerPositionAndLook {
    pub x: MCDouble,
    pub y: MCDouble,
    pub z: MCDouble,
    pub yaw: MCFloat,
    pub pitch: MCFloat,
    pub flags: MCByte,
}
impl Into<Vec<u8>> for PlayerPositionAndLook {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x08).into(); // 0x08 Player Position and Look.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.x));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.y));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.z));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.yaw));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.pitch));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.flags));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for PlayerPositionAndLook {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for PlayerPositionAndLook {
    fn new() -> Self {
        PlayerPositionAndLook {
            x: 0.0.into(),
            y: 0.0.into(),
            z: 0.0.into(),
            yaw: 0.0.into(),
            pitch: 0.0.into(),
            flags: 0x00.into(),
        }
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut playerpositionandlook = PlayerPositionAndLook::new();
        playerpositionandlook.x = MCDouble::read(t).await?;
        playerpositionandlook.y = MCDouble::read(t).await?;
        playerpositionandlook.z = MCDouble::read(t).await?;
        playerpositionandlook.yaw = MCFloat::read(t).await?;
        playerpositionandlook.pitch = MCFloat::read(t).await?;
        playerpositionandlook.flags = MCByte::read(t).await?;
        Ok(playerpositionandlook)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

// TODO: Actually send the position.
#[derive(Debug, Clone)]
pub struct SpawnPosition {
    pub position: MCPosition,
}
impl Into<Vec<u8>> for SpawnPosition {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x05).into(); // 0x05 Spawn Position.
                                                             // temp.extend_from_slice(&Into::<Vec<u8>>::into(self.position));
        temp.extend_from_slice(&0u64.to_be_bytes());
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for SpawnPosition {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for SpawnPosition {
    fn new() -> Self {
        SpawnPosition {
            position: MCPosition::new(),
        }
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut spawnposition = SpawnPosition::new();
        spawnposition.position = MCPosition::read(t).await?;
        Ok(spawnposition)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KeepAlivePing {
    pub payload: MCVarInt,
}
impl Into<Vec<u8>> for KeepAlivePing {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x00).into(); // 0x00 Keep Alive.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.payload));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for KeepAlivePing {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for KeepAlivePing {
    fn new() -> Self {
        KeepAlivePing { payload: 0.into() }
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut keepalive = KeepAlivePing::new();
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
pub struct Disconnect {
    pub reason: MCChat,
}
impl Into<Vec<u8>> for Disconnect {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x40).into(); // 0x40 Disconnect.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.reason));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for Disconnect {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for Disconnect {
    fn new() -> Self {
        Disconnect {
            reason: MCChat {
                text: "Disconnected".into(),
            },
        }
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut keepalive = Disconnect::new();
        keepalive.reason = MCChat::read(t).await?;
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
pub struct ClientboundChatMessage {
    pub text: MCChat,
    pub position: MCByte, // 0: chat (chat box), 1: system message (chat box), 2: above hotbar
}
impl Into<Vec<u8>> for ClientboundChatMessage {
    fn into(self) -> Vec<u8> {
        let mut out = vec![];
        let mut temp: Vec<u8> = MCVarInt::from(0x02).into(); // 0x02 Clientbound Chat Message.
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.text));
        temp.extend_from_slice(&Into::<Vec<u8>>::into(self.position));
        out.extend_from_slice(&Into::<Vec<u8>>::into(MCVarInt::from(temp.len() as i32)));
        out.extend_from_slice(&temp);
        out
    }
}
impl TryFrom<Vec<u8>> for ClientboundChatMessage {
    type Error = &'static str;
    fn try_from(_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Err("unimplemented")
    }
}
#[async_trait::async_trait]
impl PacketCommon for ClientboundChatMessage {
    fn new() -> Self {
        ClientboundChatMessage {
            text: MCChat { text: "".into() },
            position: 0.into(),
        }
    }
    async fn read(t: &mut TcpStream) -> tokio::io::Result<Self> {
        let mut clientboundchatmessage = ClientboundChatMessage::new();
        clientboundchatmessage.text = MCChat::read(t).await?;
        clientboundchatmessage.position = MCByte::read(t).await?;
        Ok(clientboundchatmessage)
    }
    async fn write(&self, t: &mut TcpStream) -> tokio::io::Result<()> {
        for b in Into::<Vec<u8>>::into(self.clone()) {
            write_byte(t, b).await?;
        }
        Ok(())
    }
}
