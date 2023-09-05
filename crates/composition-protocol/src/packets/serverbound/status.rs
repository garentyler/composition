use bytes::Bytes;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SS00StatusRequest;
crate::packets::packet!(
    SS00StatusRequest,
    0x00,
    crate::ClientState::Status,
    true,
    |_data: &mut Bytes| -> composition_parsing::Result<SS00StatusRequest> { Ok(SS00StatusRequest) },
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
    |data: &mut Bytes| -> composition_parsing::Result<SS01PingRequest> {
        Ok(SS01PingRequest {
            payload: i64::parse(data)?,
        })
    },
    |packet: &SS01PingRequest| -> Vec<u8> { packet.payload.serialize() }
);
