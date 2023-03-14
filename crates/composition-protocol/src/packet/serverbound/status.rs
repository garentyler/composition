use crate::packet::{GenericPacket, Packet, PacketId};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SS00StatusRequest;
impl Packet for SS00StatusRequest {
    fn id() -> PacketId {
        0x00
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Status
    }
    fn serverbound() -> bool {
        true
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        Ok((data, SS00StatusRequest))
    }
    fn serialize_body(&self) -> Vec<u8> {
        vec![]
    }
}
impl From<SS00StatusRequest> for GenericPacket {
    fn from(value: SS00StatusRequest) -> Self {
        GenericPacket::SS00StatusRequest(value)
    }
}
impl TryFrom<GenericPacket> for SS00StatusRequest {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::SS00StatusRequest(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SS01PingRequest {
    payload: i64,
}
impl Packet for SS01PingRequest {
    fn id() -> PacketId {
        0x01
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Status
    }
    fn serverbound() -> bool {
        true
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, payload) = nom::number::streaming::be_i64(data)?;
        Ok((data, SS01PingRequest { payload }))
    }
    fn serialize_body(&self) -> Vec<u8> {
        self.payload.to_be_bytes().to_vec()
    }
}
impl From<SS01PingRequest> for GenericPacket {
    fn from(value: SS01PingRequest) -> Self {
        GenericPacket::SS01PingRequest(value)
    }
}
impl TryFrom<GenericPacket> for SS01PingRequest {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::SS01PingRequest(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}
