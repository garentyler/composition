use crate::{util::*, ProtocolError};
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Clone, Debug, PartialEq)]
pub struct SP08CommandSuggestionsRequest {
    pub transaction_id: i32,
    pub text: String,
}
crate::packet::packet!(
    SP08CommandSuggestionsRequest,
    0x08,
    crate::ClientState::Play,
    true,
    |data: &'data [u8]| -> ParseResult<'data, SP08CommandSuggestionsRequest> {
        let (data, transaction_id) = parse_varint(data)?;
        let (data, text) = parse_string(data)?;
        Ok((
            data,
            SP08CommandSuggestionsRequest {
                transaction_id,
                text,
            },
        ))
    },
    |packet: &SP08CommandSuggestionsRequest| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(packet.transaction_id));
        output.extend_from_slice(&serialize_string(&packet.text));
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP11KeepAlive {
    pub payload: i64,
}
crate::packet::packet!(
    SP11KeepAlive,
    0x11,
    crate::ClientState::Play,
    true,
    |data: &'data [u8]| -> ParseResult<'data, SP11KeepAlive> {
        let (data, mut bytes) = take_bytes(8)(data)?;
        let payload = bytes
            .read_i64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        Ok((data, SP11KeepAlive { payload }))
    },
    |packet: &SP11KeepAlive| -> Vec<u8> { packet.payload.to_be_bytes().to_vec() }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP13SetPlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}
crate::packet::packet!(
    SP13SetPlayerPosition,
    0x13,
    crate::ClientState::Play,
    true,
    |data: &'data [u8]| -> ParseResult<'data, SP13SetPlayerPosition> {
        let (data, mut bytes) = take_bytes(8)(data)?;
        let x = bytes
            .read_f64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(8)(data)?;
        let y = bytes
            .read_f64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(8)(data)?;
        let z = bytes
            .read_f64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, on_ground) = take_bytes(1usize)(data)?;
        let on_ground = on_ground == [0x01];
        Ok((data, SP13SetPlayerPosition { x, y, z, on_ground }))
    },
    |packet: &SP13SetPlayerPosition| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&packet.x.to_be_bytes());
        output.extend_from_slice(&packet.y.to_be_bytes());
        output.extend_from_slice(&packet.z.to_be_bytes());
        output.push(if packet.on_ground { 0x01 } else { 0x00 });
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP14SetPlayerPositionAndRotation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
crate::packet::packet!(
    SP14SetPlayerPositionAndRotation,
    0x14,
    crate::ClientState::Play,
    true,
    |data: &'data [u8]| -> ParseResult<'data, SP14SetPlayerPositionAndRotation> {
        let (data, mut bytes) = take_bytes(8)(data)?;
        let x = bytes
            .read_f64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(8)(data)?;
        let y = bytes
            .read_f64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(8)(data)?;
        let z = bytes
            .read_f64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(4)(data)?;
        let yaw = bytes
            .read_f32::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(4)(data)?;
        let pitch = bytes
            .read_f32::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, on_ground) = take_bytes(1usize)(data)?;
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
    },
    |packet: &SP14SetPlayerPositionAndRotation| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&packet.x.to_be_bytes());
        output.extend_from_slice(&packet.y.to_be_bytes());
        output.extend_from_slice(&packet.z.to_be_bytes());
        output.extend_from_slice(&packet.yaw.to_be_bytes());
        output.extend_from_slice(&packet.pitch.to_be_bytes());
        output.push(if packet.on_ground { 0x01 } else { 0x00 });
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP15SetPlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
crate::packet::packet!(
    SP15SetPlayerRotation,
    0x15,
    crate::ClientState::Play,
    true,
    |data: &'data [u8]| -> ParseResult<'data, SP15SetPlayerRotation> {
        let (data, mut bytes) = take_bytes(4)(data)?;
        let yaw = bytes
            .read_f32::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(4)(data)?;
        let pitch = bytes
            .read_f32::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, on_ground) = take_bytes(1usize)(data)?;
        let on_ground = on_ground == [0x01];
        Ok((
            data,
            SP15SetPlayerRotation {
                yaw,
                pitch,
                on_ground,
            },
        ))
    },
    |packet: &SP15SetPlayerRotation| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&packet.yaw.to_be_bytes());
        output.extend_from_slice(&packet.pitch.to_be_bytes());
        output.push(if packet.on_ground { 0x01 } else { 0x00 });
        output
    }
);
