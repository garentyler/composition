use crate::mctypes::Chat;
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq)]
pub struct CS00StatusResponse {
    pub response: Chat,
}
crate::packets::packet!(
    CS00StatusResponse,
    0x00,
    crate::ClientState::Status,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CS00StatusResponse> {
        Ok(CS00StatusResponse {
            response: Chat::parse(data)?,
        })
    },
    |packet: &CS00StatusResponse| -> Vec<u8> { packet.response.serialize() }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CS01PingResponse {
    pub payload: i64,
}
crate::packets::packet!(
    CS01PingResponse,
    0x01,
    crate::ClientState::Status,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CS01PingResponse> {
        Ok(CS01PingResponse {
            payload: i64::parse(data)?,
        })
    },
    |packet: &CS01PingResponse| -> Vec<u8> { packet.payload.serialize() }
);
