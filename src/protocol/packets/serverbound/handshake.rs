use crate::protocol::{ClientState, mctypes::VarInt};

#[derive(Clone, Debug, PartialEq)]
pub struct SH00Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: ClientState,
}
crate::protocol::packets::packet!(
    SH00Handshake,
    0x00,
    ClientState::Handshake,
    true,
    |data: &'data [u8]| -> crate::protocol::parsing::ParseResult<'data, SH00Handshake> {
        let (data, protocol_version) = VarInt::parse(data)?;
        let (data, server_address) = String::parse(data)?;
        let (data, server_port) = u16::parse(data)?;
        let (data, next_state) = VarInt::parse(data)?;

        Ok((data, SH00Handshake {
            protocol_version,
            server_address,
            server_port,
            next_state: match *next_state {
                1 => ClientState::Status,
                2 => ClientState::Login,
                _ => todo!("Invalid next state"),
            },
        }))
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
                _ => panic!("invalid SH00Handshake next_state"),
            })
            .serialize(),
        );
        output
    }
);
