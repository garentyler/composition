use crate::{util::*, ProtocolError};
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SS00StatusRequest;
crate::packet::packet!(
    SS00StatusRequest,
    0x00,
    crate::ClientState::Status,
    true,
    |data: &'data [u8]| -> ParseResult<'data, SS00StatusRequest> { Ok((data, SS00StatusRequest)) },
    |_packet: &SS00StatusRequest| -> Vec<u8> { vec![] }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SS01PingRequest {
    pub payload: i64,
}
crate::packet::packet!(
    SS01PingRequest,
    0x01,
    crate::ClientState::Status,
    true,
    |data: &'data [u8]| -> ParseResult<'data, SS01PingRequest> {
        let (data, mut bytes) = take_bytes(8)(data)?;
        let payload = bytes
            .read_i64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        Ok((data, SS01PingRequest { payload }))
    },
    |packet: &SS01PingRequest| -> Vec<u8> { packet.payload.to_be_bytes().to_vec() }
);
