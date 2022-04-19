use super::NetworkClientState;
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
    // Play
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
            Login => unimplemented!("Parse Login packet"),
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
                serialize_json(serde_json::json!({
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
            _ => unimplemented!(),
        };
        let mut id_and_body = serialize_varint(id as i32);
        id_and_body.append(&mut body);
        let mut output = serialize_varint(id_and_body.len() as i32);
        output.append(&mut id_and_body);
        output
    }
}
