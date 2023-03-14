use crate::{
    packet::{GenericPacket, Packet, PacketId},
    util::{parse_string, parse_varint, serialize_string, serialize_varint},
};

#[derive(Clone, Debug, PartialEq)]
pub struct SP08CommandSuggestionsRequest {
    transaction_id: i32,
    text: String,
}
impl Packet for SP08CommandSuggestionsRequest {
    fn id() -> PacketId {
        0x08
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        true
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, transaction_id) = parse_varint(data)?;
        let (data, text) = parse_string(data)?;
        Ok((
            data,
            SP08CommandSuggestionsRequest {
                transaction_id,
                text,
            },
        ))
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(self.transaction_id));
        output.extend_from_slice(&serialize_string(&self.text));
        output
    }
}
impl From<SP08CommandSuggestionsRequest> for GenericPacket {
    fn from(value: SP08CommandSuggestionsRequest) -> Self {
        GenericPacket::SP08CommandSuggestionsRequest(value)
    }
}
impl TryFrom<GenericPacket> for SP08CommandSuggestionsRequest {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::SP08CommandSuggestionsRequest(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP11KeepAlive {
    payload: i64,
}
impl Packet for SP11KeepAlive {
    fn id() -> PacketId {
        0x11
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        true
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, payload) = nom::number::streaming::be_i64(data)?;
        Ok((data, SP11KeepAlive { payload }))
    }
    fn serialize_body(&self) -> Vec<u8> {
        self.payload.to_be_bytes().to_vec()
    }
}
impl From<SP11KeepAlive> for GenericPacket {
    fn from(value: SP11KeepAlive) -> Self {
        GenericPacket::SP11KeepAlive(value)
    }
}
impl TryFrom<GenericPacket> for SP11KeepAlive {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::SP11KeepAlive(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP13SetPlayerPosition {
    x: f64,
    y: f64,
    z: f64,
    on_ground: bool,
}
impl Packet for SP13SetPlayerPosition {
    fn id() -> PacketId {
        0x13
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        true
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, x) = nom::number::streaming::be_f64(data)?;
        let (data, y) = nom::number::streaming::be_f64(data)?;
        let (data, z) = nom::number::streaming::be_f64(data)?;
        let (data, on_ground) = nom::bytes::streaming::take(1usize)(data)?;
        let on_ground = on_ground == [0x01];
        Ok((data, SP13SetPlayerPosition { x, y, z, on_ground }))
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&self.x.to_be_bytes());
        output.extend_from_slice(&self.y.to_be_bytes());
        output.extend_from_slice(&self.z.to_be_bytes());
        output.push(if self.on_ground { 0x01 } else { 0x00 });
        output
    }
}
impl From<SP13SetPlayerPosition> for GenericPacket {
    fn from(value: SP13SetPlayerPosition) -> Self {
        GenericPacket::SP13SetPlayerPosition(value)
    }
}
impl TryFrom<GenericPacket> for SP13SetPlayerPosition {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::SP13SetPlayerPosition(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP14SetPlayerPositionAndRotation {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}
impl Packet for SP14SetPlayerPositionAndRotation {
    fn id() -> PacketId {
        0x14
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        true
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, x) = nom::number::streaming::be_f64(data)?;
        let (data, y) = nom::number::streaming::be_f64(data)?;
        let (data, z) = nom::number::streaming::be_f64(data)?;
        let (data, yaw) = nom::number::streaming::be_f32(data)?;
        let (data, pitch) = nom::number::streaming::be_f32(data)?;
        let (data, on_ground) = nom::bytes::streaming::take(1usize)(data)?;
        let on_ground = on_ground == [0x01];
        Ok((
            data,
            SP14SetPlayerPositionAndRotation {
                x,
                y,
                z,
                yaw,
                pitch,
                on_ground,
            },
        ))
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&self.x.to_be_bytes());
        output.extend_from_slice(&self.y.to_be_bytes());
        output.extend_from_slice(&self.z.to_be_bytes());
        output.extend_from_slice(&self.yaw.to_be_bytes());
        output.extend_from_slice(&self.pitch.to_be_bytes());
        output.push(if self.on_ground { 0x01 } else { 0x00 });
        output
    }
}
impl From<SP14SetPlayerPositionAndRotation> for GenericPacket {
    fn from(value: SP14SetPlayerPositionAndRotation) -> Self {
        GenericPacket::SP14SetPlayerPositionAndRotation(value)
    }
}
impl TryFrom<GenericPacket> for SP14SetPlayerPositionAndRotation {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::SP14SetPlayerPositionAndRotation(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP15SetPlayerRotation {
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}
impl Packet for SP15SetPlayerRotation {
    fn id() -> PacketId {
        0x15
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        true
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, yaw) = nom::number::streaming::be_f32(data)?;
        let (data, pitch) = nom::number::streaming::be_f32(data)?;
        let (data, on_ground) = nom::bytes::streaming::take(1usize)(data)?;
        let on_ground = on_ground == [0x01];
        Ok((
            data,
            SP15SetPlayerRotation {
                yaw,
                pitch,
                on_ground,
            },
        ))
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&self.yaw.to_be_bytes());
        output.extend_from_slice(&self.pitch.to_be_bytes());
        output.push(if self.on_ground { 0x01 } else { 0x00 });
        output
    }
}
impl From<SP15SetPlayerRotation> for GenericPacket {
    fn from(value: SP15SetPlayerRotation) -> Self {
        GenericPacket::SP15SetPlayerRotation(value)
    }
}
impl TryFrom<GenericPacket> for SP15SetPlayerRotation {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::SP15SetPlayerRotation(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}
