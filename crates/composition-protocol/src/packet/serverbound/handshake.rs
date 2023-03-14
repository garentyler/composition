use crate::{
    packet::{GenericPacket, Packet, PacketId},
    util::{parse_string, parse_varint, serialize_string, serialize_varint},
    ClientState,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SH00Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: ClientState,
}
impl Packet for SH00Handshake {
    fn id() -> PacketId {
        0x00
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Handshake
    }
    fn serverbound() -> bool {
        true
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, protocol_version) = parse_varint(data)?;
        let (data, server_address) = parse_string(data)?;
        let (data, server_port) = nom::number::streaming::be_u16(data)?;
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
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&self.protocol_version.to_be_bytes());
        output.extend_from_slice(&serialize_string(&self.server_address));
        output.extend_from_slice(&self.server_port.to_be_bytes());
        output.extend_from_slice(&serialize_varint(match self.next_state {
            ClientState::Status => 0x01,
            ClientState::Login => 0x02,
            _ => panic!("invalid SH00Handshake next_state"),
        }));
        output
    }
}
impl From<SH00Handshake> for GenericPacket {
    fn from(value: SH00Handshake) -> Self {
        GenericPacket::SH00Handshake(value)
    }
}
impl TryFrom<GenericPacket> for SH00Handshake {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::SH00Handshake(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}
