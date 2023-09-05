use crate::{
    entities::{EntityPosition, EntityRotation},
    mctypes::VarInt,
};
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq)]
pub struct SP08CommandSuggestionsRequest {
    pub transaction_id: VarInt,
    pub text: String,
}
crate::packets::packet!(
    SP08CommandSuggestionsRequest,
    0x08,
    crate::ClientState::Play,
    true,
    |data: &mut Bytes| -> composition_parsing::Result<SP08CommandSuggestionsRequest> {
        Ok(SP08CommandSuggestionsRequest {
            transaction_id: VarInt::parse(data)?,
            text: String::parse(data)?,
        })
    },
    |packet: &SP08CommandSuggestionsRequest| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.transaction_id.serialize());
        output.extend(packet.text.serialize());
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP11KeepAlive {
    pub payload: i64,
}
crate::packets::packet!(
    SP11KeepAlive,
    0x11,
    crate::ClientState::Play,
    true,
    |data: &mut Bytes| -> composition_parsing::Result<SP11KeepAlive> {
        Ok(SP11KeepAlive {
            payload: i64::parse(data)?,
        })
    },
    |packet: &SP11KeepAlive| -> Vec<u8> { packet.payload.serialize() }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP13SetPlayerPosition {
    pub position: EntityPosition,
    pub on_ground: bool,
}
crate::packets::packet!(
    SP13SetPlayerPosition,
    0x13,
    crate::ClientState::Play,
    true,
    |data: &mut Bytes| -> composition_parsing::Result<SP13SetPlayerPosition> {
        Ok(SP13SetPlayerPosition {
            position: EntityPosition::parse(data)?,
            on_ground: bool::parse(data)?,
        })
    },
    |packet: &SP13SetPlayerPosition| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.position.serialize());
        output.extend(packet.on_ground.serialize());
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP14SetPlayerPositionAndRotation {
    pub position: EntityPosition,
    pub rotation: EntityRotation,
    pub on_ground: bool,
}
crate::packets::packet!(
    SP14SetPlayerPositionAndRotation,
    0x14,
    crate::ClientState::Play,
    true,
    |data: &mut Bytes| -> composition_parsing::Result<SP14SetPlayerPositionAndRotation> {
        Ok(SP14SetPlayerPositionAndRotation {
            position: EntityPosition::parse(data)?,
            rotation: EntityRotation::parse(data)?,
            on_ground: bool::parse(data)?,
        })
    },
    |packet: &SP14SetPlayerPositionAndRotation| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.position.serialize());
        output.extend(packet.rotation.serialize());
        output.extend(packet.on_ground.serialize());
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP15SetPlayerRotation {
    pub rotation: EntityRotation,
    pub on_ground: bool,
}
crate::packets::packet!(
    SP15SetPlayerRotation,
    0x15,
    crate::ClientState::Play,
    true,
    |data: &mut Bytes| -> composition_parsing::Result<SP15SetPlayerRotation> {
        Ok(SP15SetPlayerRotation {
            rotation: EntityRotation::parse(data)?,
            on_ground: bool::parse(data)?,
        })
    },
    |packet: &SP15SetPlayerRotation| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.rotation.serialize());
        output.extend(packet.on_ground.serialize());
        output
    }
);
