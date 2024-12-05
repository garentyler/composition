use crate::protocol::types::Json;

#[derive(Clone, Debug, PartialEq)]
pub struct CS00StatusResponse {
    pub response: Json,
}
crate::protocol::packets::packet!(
    CS00StatusResponse,
    0x00,
    crate::protocol::ClientState::Status,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CS00StatusResponse> {
        let (data, response) = Json::parse(data)?;
        Ok((data, CS00StatusResponse { response }))
    },
    |packet: &CS00StatusResponse| -> Vec<u8> { packet.response.serialize() }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CS01PingResponse {
    pub payload: i64,
}
crate::protocol::packets::packet!(
    CS01PingResponse,
    0x01,
    crate::protocol::ClientState::Status,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CS01PingResponse> {
        let (data, payload) = i64::parse(data)?;
        Ok((data, CS01PingResponse { payload }))
    },
    |packet: &CS01PingResponse| -> Vec<u8> { packet.payload.serialize() }
);
