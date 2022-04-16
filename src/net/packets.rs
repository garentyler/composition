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
        json_response: JSON,
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
        length: usize,
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
                        0 => NetworkClientState::Status,
                        1 => NetworkClientState::Login,
                        _ => return Err(ParseError::InvalidData),
                    };
                    Ok((
                        Packet::SH00Handshake {
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
                        unimplemented!("Parse SS00Request")
                    } else {
                        unimplemented!("Parse CS00Response")
                    }
                }
                0x01 => {
                    if serverbound {
                        unimplemented!("Parse SS01Ping")
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
            CS00Response { json_response } => (0x00, serialize_json(json_response.clone())),
            CS01Pong { payload } => (0x01, serialize_long(payload.clone()).to_vec()),
            _ => unimplemented!(),
        };
        let mut id_and_body = serialize_varint(id as i32);
        id_and_body.append(&mut body);
        let mut output = serialize_varint(id_and_body.len() as i32);
        output.append(&mut id_and_body);
        output
    }
}
