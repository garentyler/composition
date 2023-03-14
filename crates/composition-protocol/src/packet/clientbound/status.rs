use crate::{
    packet::{GenericPacket, Packet, PacketId},
    util::{parse_json, serialize_json},
    Json,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CS00StatusResponse {
    response: Json,
}
impl Packet for CS00StatusResponse {
    fn id() -> PacketId {
        0x00
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Status
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, response) = parse_json(data)?;
        Ok((data, CS00StatusResponse { response }))
    }
    fn serialize_body(&self) -> Vec<u8> {
        serialize_json(&self.response)
    }
}
impl From<CS00StatusResponse> for GenericPacket {
    fn from(value: CS00StatusResponse) -> Self {
        GenericPacket::CS00StatusResponse(value)
    }
}
impl TryFrom<GenericPacket> for CS00StatusResponse {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CS00StatusResponse(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CS01PingResponse {
    payload: i64,
}
impl Packet for CS01PingResponse {
    fn id() -> PacketId {
        0x01
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Status
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, payload) = nom::number::streaming::be_i64(data)?;
        Ok((data, CS01PingResponse { payload }))
    }
    fn serialize_body(&self) -> Vec<u8> {
        self.payload.to_be_bytes().to_vec()
    }
}
impl From<CS01PingResponse> for GenericPacket {
    fn from(value: CS01PingResponse) -> Self {
        GenericPacket::CS01PingResponse(value)
    }
}
impl TryFrom<GenericPacket> for CS01PingResponse {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CS01PingResponse(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}
