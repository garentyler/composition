use crate::{util::*, ClientState, ProtocolError};
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Clone, Debug, PartialEq)]
pub struct SH00Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: ClientState,
}
crate::packet::packet!(
    SH00Handshake,
    0x00,
    ClientState::Handshake,
    true,
    |data: &'data [u8]| -> ParseResult<'data, SH00Handshake> {
        let (data, protocol_version) = parse_varint(data)?;
        let (data, server_address) = parse_string(data)?;
        let (data, mut bytes) = take_bytes(2)(data)?;
        let server_port = bytes
            .read_u16::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, next_state) = parse_varint(data)?;

        Ok((
            data,
            SH00Handshake {
                protocol_version,
                server_address,
                server_port,
                next_state: match next_state {
                    1 => ClientState::Status,
                    2 => ClientState::Login,
                    _ => todo!("Invalid next state"),
                },
            },
        ))
    },
    |packet: &SH00Handshake| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&packet.protocol_version.to_be_bytes());
        output.extend_from_slice(&serialize_string(&packet.server_address));
        output.extend_from_slice(&packet.server_port.to_be_bytes());
        output.extend_from_slice(&serialize_varint(match packet.next_state {
            ClientState::Status => 0x01,
            ClientState::Login => 0x02,
            _ => panic!("invalid SH00Handshake next_state"),
        }));
        output
    }
);
