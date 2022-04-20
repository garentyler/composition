use super::{mctypes::*, NetworkClientState};
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Packet {
    // Handshake
    SH00Handshake {
        protocol_version: i32,
        server_address: String,
        server_port: u16,
        next_state: NetworkClientState,
    },

    // Status
    CS00Response {
        version_name: String,
        protocol_version: i32,
        max_players: usize,
        current_players: usize,
        description: JSON,
    },
    CS01Pong {
        payload: i64,
    },
    SS00Request,
    SS01Ping {
        payload: i64,
    },

    // Login
    CL00Disconnect {
        reason: JSON,
    },
    CL01EncryptionRequest {
        server_id: String,
        public_key: Vec<u8>,
        verify_token: Vec<u8>,
    },
    CL02LoginSuccess {
        uuid: u128,
        username: String,
    },
    CL03SetCompression {
        threshold: usize,
    },
    CL04LoginPluginRequest {
        message_id: i32,
        channel: String,
        data: Vec<u8>,
    },
    SL00LoginStart {
        username: String,
    },
    SL01EncryptionResponse {
        shared_secret: Vec<u8>,
        verify_token: Vec<u8>,
    },
    SL02LoginPluginResponse {
        message_id: i32,
        successful: bool,
        data: Option<Vec<u8>>,
    },

    // Play
    CP14WindowItems {
        window_id: u8,
        state_id: i32,
        slots: Vec<NBT>,
        carried_item: NBT,
    },
    CP26JoinGame,
    CP48HeldItemChange,
    CP66DeclareRecipes,
    CP67Tags,
    CP1BEntityStatus,
    CP12DeclareCommands,
    CP39UnlockRecipes,
    CP22ChunkDataAndUpdateLight,
    CP38PlayerPositionAndLook {
        x: (f64, bool),
        y: (f64, bool),
        z: (f64, bool),
        yaw: (f32, bool),
        pitch: (f32, bool),
        teleport_id: i32,
        dismount_vehicle: bool,
    },
    CP36PlayerInfo,
    CP49UpdateViewPosition,
    CP25UpdateLight,
    CP4BSpawnPosition {
        location: Position,
        angle: f32,
    },
    CP00TeleportConfirm,

    SP05ClientSettings,
    SP04ClientStatus,
}
impl Packet {
    pub fn parse_body(
        data: &[u8],
        _length: usize,
        id: usize,
        state: NetworkClientState,
        serverbound: bool,
    ) -> ParseResult<Packet> {
        use NetworkClientState::*;
        use Packet::*;

        let mut offset = 0;
        match state {
            Disconnected => Err(ParseError::InvalidData),
            Handshake => {
                if id == 0x00 && serverbound {
                    let (protocol_version, offset_delta) = parse_varint(&data[offset..])?;
                    offset += offset_delta;
                    let (server_address, offset_delta) = parse_string(&data[offset..])?;
                    offset += offset_delta;
                    let (server_port, offset_delta) = parse_unsigned_short(&data[offset..])?;
                    offset += offset_delta;
                    let (next_state, offset_delta) = parse_varint(&data[offset..])?;
                    offset += offset_delta;
                    let next_state = match next_state {
                        1 => NetworkClientState::Status,
                        2 => NetworkClientState::Login,
                        _ => return Err(ParseError::InvalidData),
                    };
                    Ok((
                        SH00Handshake {
                            protocol_version,
                            server_address,
                            server_port,
                            next_state,
                        },
                        offset,
                    ))
                } else {
                    Err(ParseError::InvalidData)
                }
            }
            Status => match id {
                0x00 => {
                    if serverbound {
                        Ok((SS00Request, offset))
                    } else {
                        unimplemented!("Parse CS00Response")
                    }
                }
                0x01 => {
                    if serverbound {
                        let (payload, offset_delta) = parse_long(&data[offset..])?;
                        offset += offset_delta;
                        Ok((SS01Ping { payload }, offset))
                    } else {
                        unimplemented!("Parse CS01Pong")
                    }
                }
                _ => Err(ParseError::InvalidData),
            },
            Login => match id {
                0x00 => {
                    if serverbound {
                        let (username, offset_delta) = parse_string(&data[offset..])?;
                        offset += offset_delta;
                        Ok((SL00LoginStart { username }, offset))
                    } else {
                        unimplemented!("Parse CL00Disconnect")
                    }
                }
                0x01 => {
                    if serverbound {
                        unimplemented!("Parse SL01EncryptionResponse")
                    } else {
                        unimplemented!("Parse CL01EncryptionRequest")
                    }
                }
                0x02 => {
                    if serverbound {
                        unimplemented!("Parse SL02LoginPluginResponse")
                    } else {
                        unimplemented!("Parse CL02LoginSuccess")
                    }
                }
                0x03 => {
                    if serverbound {
                        Err(ParseError::InvalidData)
                    } else {
                        unimplemented!("Parse CL03SetCompression")
                    }
                }
                0x04 => {
                    if serverbound {
                        Err(ParseError::InvalidData)
                    } else {
                        unimplemented!("Parse CL04LoginPluginRequest")
                    }
                }
                _ => Err(ParseError::InvalidData),
            },
            Play => unimplemented!("Parse Play packet"),
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        use Packet::*;
        let (id, mut body): (usize, Vec<u8>) = match self {
            CS00Response {
                version_name,
                protocol_version,
                max_players,
                current_players,
                description,
            } => (
                0x00,
                serialize_json(json!({
                    "version": {
                        "name": version_name,
                        "protocol": protocol_version,
                    },
                    "players": {
                        "max": max_players,
                        "online": current_players,
                    },
                    "description": description,
                    // TODO: Add base64 favicon
                    "favicon": format!("data:image/png;base64,{}", radix64::STD_NO_PAD.encode(FAVICON.as_ref().unwrap())),
                })),
            ),
            CS01Pong { payload } => (0x01, serialize_long(*payload).to_vec()),
            CL00Disconnect { reason } => (0x00, serialize_json(reason.clone())),
            CL02LoginSuccess { uuid, username } => (0x02, {
                let mut out = vec![];
                out.extend(uuid.to_be_bytes());
                out.extend(serialize_string(username));
                out
            }),
            CP14WindowItems {
                window_id,
                state_id,
                slots,
                carried_item,
            } => (0x14, {
                let mut out = vec![*window_id];
                out.extend(serialize_varint(*state_id));
                out.extend(serialize_varint(slots.len() as i32));
                for slot in slots {
                    out.extend(serialize_nbt(slot.clone()));
                }
                out.extend(serialize_nbt(carried_item.clone()));
                out
            }),
            CP38PlayerPositionAndLook {
                x,
                y,
                z,
                yaw,
                pitch,
                teleport_id,
                dismount_vehicle,
            } => (0x38, {
                let mut out = vec![];
                out.extend(serialize_double(x.0));
                out.extend(serialize_double(y.0));
                out.extend(serialize_double(z.0));
                out.extend(serialize_float(yaw.0));
                out.extend(serialize_float(pitch.0));
                let mut flags = 0x00;
                if x.1 {
                    flags |= 0x01;
                }
                if y.1 {
                    flags |= 0x02;
                }
                if z.1 {
                    flags |= 0x04;
                }
                if yaw.1 {
                    flags |= 0x10;
                }
                if pitch.1 {
                    flags |= 0x08;
                }
                out.push(flags);
                out.extend(serialize_varint(*teleport_id));
                out.extend(serialize_bool(*dismount_vehicle));
                out
            }),
            CP4BSpawnPosition { location, angle } => (0x4b, {
                let mut out = vec![];
                out.extend(location.serialize());
                out.extend(serialize_float(*angle));
                out
            }),
            _ => unimplemented!("Serializing unknown packet"),
        };
        let mut id_and_body = serialize_varint(id as i32);
        id_and_body.append(&mut body);
        let mut output = serialize_varint(id_and_body.len() as i32);
        output.append(&mut id_and_body);
        output
    }
}
