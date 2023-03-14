pub mod clientbound;
pub mod serverbound;

use crate::ClientState;

pub type PacketId = i32;

pub trait Packet: TryFrom<GenericPacket> + Into<GenericPacket> + std::fmt::Debug {
    fn id() -> PacketId;
    fn client_state() -> crate::ClientState;
    fn serverbound() -> bool;

    // The slice given should only be the exact amount of data in the body.
    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized;
    fn serialize_body(&self) -> Vec<u8>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenericPacket {
    // Handshake
    SH00Handshake(serverbound::SH00Handshake),

    // Status
    SS00StatusRequest(serverbound::SS00StatusRequest),
    SS01PingRequest(serverbound::SS01PingRequest),

    CS00StatusResponse(clientbound::CS00StatusResponse),
    CS01PingResponse(clientbound::CS01PingResponse),

    // Login
    SL00LoginStart(serverbound::SL00LoginStart),
    SL01EncryptionResponse(serverbound::SL01EncryptionResponse),
    SL02LoginPluginResponse(serverbound::SL02LoginPluginResponse),

    CL00Disconnect(clientbound::CL00Disconnect),
    CL01EncryptionRequest(clientbound::CL01EncryptionRequest),
    CL02LoginSuccess(clientbound::CL02LoginSuccess),
    CL03SetCompression(clientbound::CL03SetCompression),
    CL04LoginPluginRequest(clientbound::CL04LoginPluginRequest),

    // Play
    SP08CommandSuggestionsRequest(serverbound::SP08CommandSuggestionsRequest),
    SP11KeepAlive(serverbound::SP11KeepAlive),
    SP13SetPlayerPosition(serverbound::SP13SetPlayerPosition),
    SP14SetPlayerPositionAndRotation(serverbound::SP14SetPlayerPositionAndRotation),
    SP15SetPlayerRotation(serverbound::SP15SetPlayerRotation),

    CP00SpawnEntity(clientbound::CP00SpawnEntity),
    CP0BChangeDifficulty(clientbound::CP0BChangeDifficulty),
    CP17Disconnect(clientbound::CP17Disconnect),
    CP1FKeepAlive(clientbound::CP1FKeepAlive),
    CP21WorldEvent(clientbound::CP21WorldEvent),
    CP50SetEntityVelocity(clientbound::CP50SetEntityVelocity),
    CP52SetExperience(clientbound::CP52SetExperience),
    CP68EntityEffect(clientbound::CP68EntityEffect),

    // Until we implement all the packets this will stay.
    UnimplementedPacket(PacketId),
}
impl GenericPacket {
    pub fn parse_uncompressed<'data>(
        client_state: &ClientState,
        serverbound: bool,
        data: &'data [u8],
    ) -> nom::IResult<&'data [u8], Self> {
        let (data, packet_length) = crate::util::parse_varint(data)?;
        let (data, packet_data) = nom::bytes::streaming::take(packet_length as usize)(data)?;

        let (packet_data, packet_id) = crate::util::parse_varint(packet_data)?;
        let (_packet_data, packet_body) =
            Self::parse_body(client_state, packet_id, serverbound, packet_data)?;

        // if !packet_data.is_empty() {
        //     println!("Packet data not empty after parsing!");
        // }

        Ok((data, packet_body))
    }
    pub fn parse_body<'data>(
        client_state: &ClientState,
        packet_id: PacketId,
        serverbound: bool,
        data: &'data [u8],
    ) -> nom::IResult<&'data [u8], Self> {
        fn mapper<P: Into<GenericPacket> + Sized>(
            (data, packet): (&[u8], P),
        ) -> (&[u8], GenericPacket) {
            (data, Into::<GenericPacket>::into(packet))
        }

        match (client_state, packet_id, serverbound) {
            // Handshake
            (ClientState::Handshake, 0x00, true) => {
                serverbound::SH00Handshake::parse_body(data).map(mapper)
            }

            // Status
            (ClientState::Status, 0x00, true) => {
                serverbound::SS00StatusRequest::parse_body(data).map(mapper)
            }
            (ClientState::Status, 0x01, true) => {
                serverbound::SS00StatusRequest::parse_body(data).map(mapper)
            }

            (ClientState::Status, 0x00, false) => {
                clientbound::CS00StatusResponse::parse_body(data).map(mapper)
            }
            (ClientState::Status, 0x01, false) => {
                clientbound::CS01PingResponse::parse_body(data).map(mapper)
            }

            // Login
            (ClientState::Login, 0x00, true) => {
                serverbound::SL00LoginStart::parse_body(data).map(mapper)
            }
            (ClientState::Login, 0x01, true) => {
                serverbound::SL01EncryptionResponse::parse_body(data).map(mapper)
            }
            (ClientState::Login, 0x02, true) => {
                serverbound::SL02LoginPluginResponse::parse_body(data).map(mapper)
            }

            (ClientState::Login, 0x00, false) => {
                clientbound::CL00Disconnect::parse_body(data).map(mapper)
            }
            (ClientState::Login, 0x01, false) => {
                clientbound::CL01EncryptionRequest::parse_body(data).map(mapper)
            }
            (ClientState::Login, 0x02, false) => {
                clientbound::CL02LoginSuccess::parse_body(data).map(mapper)
            }
            (ClientState::Login, 0x03, false) => {
                clientbound::CL03SetCompression::parse_body(data).map(mapper)
            }
            (ClientState::Login, 0x04, false) => {
                clientbound::CL04LoginPluginRequest::parse_body(data).map(mapper)
            }

            // Play
            (ClientState::Play, 0x08, true) => {
                serverbound::SP08CommandSuggestionsRequest::parse_body(data).map(mapper)
            }
            (ClientState::Play, 0x11, true) => {
                serverbound::SP11KeepAlive::parse_body(data).map(mapper)
            }
            (ClientState::Play, 0x13, true) => {
                serverbound::SP13SetPlayerPosition::parse_body(data).map(mapper)
            }
            (ClientState::Play, 0x14, true) => {
                serverbound::SP14SetPlayerPositionAndRotation::parse_body(data).map(mapper)
            }
            (ClientState::Play, 0x15, true) => {
                serverbound::SP15SetPlayerRotation::parse_body(data).map(mapper)
            }

            (ClientState::Play, 0x00, false) => {
                clientbound::CP00SpawnEntity::parse_body(data).map(mapper)
            }
            (ClientState::Play, 0x0b, false) => {
                clientbound::CP0BChangeDifficulty::parse_body(data).map(mapper)
            }
            (ClientState::Play, 0x17, false) => {
                clientbound::CP17Disconnect::parse_body(data).map(mapper)
            }
            (ClientState::Play, 0x1f, false) => {
                clientbound::CP1FKeepAlive::parse_body(data).map(mapper)
            }
            (ClientState::Play, 0x21, false) => {
                clientbound::CP21WorldEvent::parse_body(data).map(mapper)
            }
            (ClientState::Play, 0x50, false) => {
                clientbound::CP50SetEntityVelocity::parse_body(data).map(mapper)
            }
            (ClientState::Play, 0x52, false) => {
                clientbound::CP52SetExperience::parse_body(data).map(mapper)
            }
            (ClientState::Play, 0x68, false) => {
                clientbound::CP68EntityEffect::parse_body(data).map(mapper)
            }

            _ => Ok((&data[0..0], GenericPacket::UnimplementedPacket(packet_id))),
            // Invalid packet
            // _ => Err(nom::Err::Failure(nom::error::Error::new(
            //     data,
            //     nom::error::ErrorKind::Verify,
            // ))),
        }
    }
    pub fn serialize(&self) -> (PacketId, Vec<u8>) {
        use GenericPacket::*;

        match self {
            // Handshake
            SH00Handshake(packet) => (serverbound::SH00Handshake::id(), packet.serialize_body()),

            // Status
            SS00StatusRequest(packet) => (
                serverbound::SS00StatusRequest::id(),
                packet.serialize_body(),
            ),
            SS01PingRequest(packet) => {
                (serverbound::SS01PingRequest::id(), packet.serialize_body())
            }

            CS00StatusResponse(packet) => (
                clientbound::CS00StatusResponse::id(),
                packet.serialize_body(),
            ),
            CS01PingResponse(packet) => {
                (clientbound::CS01PingResponse::id(), packet.serialize_body())
            }

            // Login
            SL00LoginStart(packet) => (serverbound::SL00LoginStart::id(), packet.serialize_body()),
            SL01EncryptionResponse(packet) => (
                serverbound::SL01EncryptionResponse::id(),
                packet.serialize_body(),
            ),
            SL02LoginPluginResponse(packet) => (
                serverbound::SL02LoginPluginResponse::id(),
                packet.serialize_body(),
            ),

            CL00Disconnect(packet) => (clientbound::CL00Disconnect::id(), packet.serialize_body()),
            CL01EncryptionRequest(packet) => (
                clientbound::CL01EncryptionRequest::id(),
                packet.serialize_body(),
            ),
            CL02LoginSuccess(packet) => {
                (clientbound::CL02LoginSuccess::id(), packet.serialize_body())
            }
            CL03SetCompression(packet) => (
                clientbound::CL03SetCompression::id(),
                packet.serialize_body(),
            ),
            CL04LoginPluginRequest(packet) => (
                clientbound::CL04LoginPluginRequest::id(),
                packet.serialize_body(),
            ),

            // Play
            SP08CommandSuggestionsRequest(packet) => (
                serverbound::SP08CommandSuggestionsRequest::id(),
                packet.serialize_body(),
            ),
            SP11KeepAlive(packet) => (serverbound::SP11KeepAlive::id(), packet.serialize_body()),
            SP13SetPlayerPosition(packet) => (
                serverbound::SP13SetPlayerPosition::id(),
                packet.serialize_body(),
            ),
            SP14SetPlayerPositionAndRotation(packet) => (
                serverbound::SP14SetPlayerPositionAndRotation::id(),
                packet.serialize_body(),
            ),
            SP15SetPlayerRotation(packet) => (
                serverbound::SP15SetPlayerRotation::id(),
                packet.serialize_body(),
            ),

            CP00SpawnEntity(packet) => {
                (clientbound::CP00SpawnEntity::id(), packet.serialize_body())
            }
            CP0BChangeDifficulty(packet) => (
                clientbound::CP0BChangeDifficulty::id(),
                packet.serialize_body(),
            ),
            CP17Disconnect(packet) => (clientbound::CP17Disconnect::id(), packet.serialize_body()),
            CP1FKeepAlive(packet) => (clientbound::CP1FKeepAlive::id(), packet.serialize_body()),
            CP21WorldEvent(packet) => (clientbound::CP21WorldEvent::id(), packet.serialize_body()),
            CP50SetEntityVelocity(packet) => (
                clientbound::CP50SetEntityVelocity::id(),
                packet.serialize_body(),
            ),
            CP52SetExperience(packet) => (
                clientbound::CP52SetExperience::id(),
                packet.serialize_body(),
            ),
            CP68EntityEffect(packet) => {
                (clientbound::CP68EntityEffect::id(), packet.serialize_body())
            }

            // Unimplemented packets get no body.
            UnimplementedPacket(packet_id) => (*packet_id, vec![]),
        }
    }
}
