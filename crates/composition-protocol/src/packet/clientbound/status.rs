use crate::{util::*, Json, ProtocolError};
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Clone, Debug, PartialEq)]
pub struct CS00StatusResponse {
    pub response: Json,
}
crate::packet::packet!(
    CS00StatusResponse,
    0x00,
    crate::ClientState::Status,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CS00StatusResponse> {
        let (data, response) = parse_json(data)?;
        Ok((data, CS00StatusResponse { response }))
    },
    |packet: &CS00StatusResponse| -> Vec<u8> { serialize_json(&packet.response) }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CS01PingResponse {
    pub payload: i64,
}
crate::packet::packet!(
    CS01PingResponse,
    0x01,
    crate::ClientState::Status,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CS01PingResponse> {
        let (data, mut bytes) = take_bytes(8)(data)?;
        let payload = bytes
            .read_i64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        Ok((data, CS01PingResponse { payload }))
    },
    |packet: &CS01PingResponse| -> Vec<u8> { packet.payload.to_be_bytes().to_vec() }
);
