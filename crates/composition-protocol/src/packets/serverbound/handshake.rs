use crate::{mctypes::VarInt, ClientState};
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq)]
pub struct SH00Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: ClientState,
}
crate::packets::packet!(
    SH00Handshake,
    0x00,
    ClientState::Handshake,
    true,
    |data: &mut Bytes| -> composition_parsing::Result<SH00Handshake> {
        Ok(SH00Handshake {
            protocol_version: VarInt::parse(data)?,
            server_address: String::parse(data)?,
            server_port: u16::parse(data)?,
            next_state: match *VarInt::parse(data)? {
                1 => ClientState::Status,
                2 => ClientState::Login,
                _ => return Err(composition_parsing::Error::InvalidData),
            },
        })
    },
    |packet: &SH00Handshake| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.protocol_version.serialize());
        output.extend(packet.server_address.serialize());
        output.extend(packet.server_port.serialize());
        output.extend(
            VarInt::from(match packet.next_state {
                ClientState::Status => 0x01,
                ClientState::Login => 0x02,
                other => panic!(
                    "Invalid next state while serializing SH00Handshake: {:?}",
                    other
                ),
            })
            .serialize(),
        );
        output
    }
);
