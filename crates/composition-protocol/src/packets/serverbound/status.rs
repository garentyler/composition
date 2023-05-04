#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SS00StatusRequest;
crate::packets::packet!(
    SS00StatusRequest,
    0x00,
    crate::ClientState::Status,
    true,
    |data: &'data [u8]| -> composition_parsing::ParseResult<'data, SS00StatusRequest> {
        Ok((data, SS00StatusRequest))
    },
    |_packet: &SS00StatusRequest| -> Vec<u8> { vec![] }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SS01PingRequest {
    pub payload: i64,
}
crate::packets::packet!(
    SS01PingRequest,
    0x01,
    crate::ClientState::Status,
    true,
    |data: &'data [u8]| -> composition_parsing::ParseResult<'data, SS01PingRequest> {
        let (data, payload) = i64::parse(data)?;
        Ok((data, SS01PingRequest { payload }))
    },
    |packet: &SS01PingRequest| -> Vec<u8> { packet.payload.serialize() }
);
