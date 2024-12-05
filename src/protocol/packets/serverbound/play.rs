use crate::protocol::{
    entities::{EntityPosition, EntityRotation},
    types::VarInt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SP08CommandSuggestionsRequest {
    pub transaction_id: VarInt,
    pub text: String,
}
crate::protocol::packets::packet!(
    SP08CommandSuggestionsRequest,
    0x08,
    crate::protocol::ClientState::Play,
    true,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], SP08CommandSuggestionsRequest> {
        let (data, transaction_id) = VarInt::parse(data)?;
        let (data, text) = String::parse(data)?;
        Ok((data, SP08CommandSuggestionsRequest {
            transaction_id,
            text,
        }))
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
crate::protocol::packets::packet!(
    SP11KeepAlive,
    0x11,
    crate::protocol::ClientState::Play,
    true,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], SP11KeepAlive> {
        let (data, payload) = i64::parse(data)?;
        Ok((data, SP11KeepAlive { payload }))
    },
    |packet: &SP11KeepAlive| -> Vec<u8> { packet.payload.serialize() }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SP13SetPlayerPosition {
    pub position: EntityPosition,
    pub on_ground: bool,
}
crate::protocol::packets::packet!(
    SP13SetPlayerPosition,
    0x13,
    crate::protocol::ClientState::Play,
    true,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], SP13SetPlayerPosition> {
        let (data, position) = EntityPosition::parse(data)?;
        let (data, on_ground) = bool::parse(data)?;
        Ok((data, SP13SetPlayerPosition {
            position,
            on_ground,
        }))
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
crate::protocol::packets::packet!(
    SP14SetPlayerPositionAndRotation,
    0x14,
    crate::protocol::ClientState::Play,
    true,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], SP14SetPlayerPositionAndRotation> {
        let (data, position) = EntityPosition::parse(data)?;
        let (data, rotation) = EntityRotation::parse(data)?;
        let (data, on_ground) = bool::parse(data)?;
        Ok((data, SP14SetPlayerPositionAndRotation {
            position,
            rotation,
            on_ground,
        }))
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
crate::protocol::packets::packet!(
    SP15SetPlayerRotation,
    0x15,
    crate::protocol::ClientState::Play,
    true,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], SP15SetPlayerRotation> {
        let (data, rotation) = EntityRotation::parse(data)?;
        let (data, on_ground) = bool::parse(data)?;
        Ok((data, SP15SetPlayerRotation {
            rotation,
            on_ground,
        }))
    },
    |packet: &SP15SetPlayerRotation| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.rotation.serialize());
        output.extend(packet.on_ground.serialize());
        output
    }
);
