pub mod clientbound;
pub mod serverbound;

pub type PacketId = i32;

pub trait Packet: std::fmt::Debug + Clone + TryFrom<GenericPacket> + Into<GenericPacket> {
    const ID: PacketId;
    const CLIENT_STATE: crate::ClientState;
    const IS_SERVERBOUND: bool;

    // The slice given should only be the exact amount of data in the body.
    fn parse_body(data: &[u8]) -> crate::util::ParseResult<'_, Self>
    where
        Self: Sized;
    fn serialize_body(&self) -> Vec<u8>;
}

macro_rules! generic_packet {
    ($($packet_type: ident),*) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum GenericPacket {
            $(
                $packet_type($packet_type),
            )*
        }
        impl GenericPacket {
            #[tracing::instrument]
            pub fn parse_uncompressed<'data>(
                client_state: crate::ClientState,
                is_serverbound: bool,
                data: &'data [u8]
            ) -> crate::util::ParseResult<'data, Self> {
                tracing::trace!(
                    "GenericPacket::parse_uncompressed: {:?} {} {:?}",
                    client_state,
                    is_serverbound,
                    data
                );

                let (data, packet_length) = crate::util::parse_varint(data)?;
                let (data, packet_data) = crate::util::take_bytes(packet_length as usize)(data)?;

                let (packet_data, packet_id) = crate::util::parse_varint(packet_data)?;
                let (_packet_data, packet_body) =
                    Self::parse_body(client_state, packet_id, is_serverbound, packet_data)?;

                // if !packet_data.is_empty() {
                //     println!("Packet data not empty after parsing!");
                // }

                Ok((data, packet_body))
            }

            #[tracing::instrument]
            pub fn parse_body<'data>(
                client_state: crate::ClientState,
                packet_id: crate::packet::PacketId,
                is_serverbound: bool,
                data: &'data [u8],
            ) -> crate::util::ParseResult<'data, Self> {
                tracing::trace!(
                    "GenericPacket::parse_body: {:?} {} {}",
                    client_state,
                    packet_id,
                    is_serverbound
                );
                match (client_state, packet_id, is_serverbound) {
                    $(
                        ($packet_type::CLIENT_STATE, $packet_type::ID, $packet_type::IS_SERVERBOUND) => $packet_type::parse_body(data).map(|(data, packet)| (data, Into::<GenericPacket>::into(packet))),
                    )*
                    _ => Ok((data, Self::UnimplementedPacket(UnimplementedPacket(packet_id)))),
                }
            }

            #[tracing::instrument]
            pub fn serialize(&self) -> (crate::packet::PacketId, Vec<u8>) {
                tracing::trace!("GenericPacket::serialize: {:?}", self);
                match self {
                    $(
                        Self::$packet_type(packet) => ($packet_type::ID, packet.serialize_body()),
                    )*
                }
            }
        }
    };
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnimplementedPacket(i32);
packet!(
    UnimplementedPacket,
    0x00,
    crate::ClientState::Disconnected,
    false,
    |data: &'data [u8]| -> crate::util::ParseResult<'data, UnimplementedPacket> {
        Ok((data, UnimplementedPacket(0i32)))
    },
    |_packet: &UnimplementedPacket| -> Vec<u8> { vec![] }
);

use clientbound::*;
use serverbound::*;
generic_packet!(
    UnimplementedPacket,
    // Handshake
    SH00Handshake,
    // Status
    SS00StatusRequest,
    SS01PingRequest,
    CS00StatusResponse,
    CS01PingResponse,
    // Login
    SL00LoginStart,
    SL01EncryptionResponse,
    SL02LoginPluginResponse,
    CL00Disconnect,
    CL01EncryptionRequest,
    CL02LoginSuccess,
    CL03SetCompression,
    CL04LoginPluginRequest,
    // Play
    SP08CommandSuggestionsRequest,
    SP11KeepAlive,
    SP13SetPlayerPosition,
    SP14SetPlayerPositionAndRotation,
    SP15SetPlayerRotation,
    CP00SpawnEntity,
    CP0BChangeDifficulty,
    CP17Disconnect,
    CP1FKeepAlive,
    CP21WorldEvent,
    CP50SetEntityVelocity,
    CP52SetExperience,
    CP68EntityEffect
);

macro_rules! packet {
    ($packet_type: ident, $id: literal, $client_state: expr, $serverbound: literal, $parse_body: expr, $serialize_body: expr) => {
        impl crate::packet::Packet for $packet_type {
            const ID: crate::packet::PacketId = $id;
            const CLIENT_STATE: crate::ClientState = $client_state;
            const IS_SERVERBOUND: bool = $serverbound;

            fn parse_body<'data>(data: &'data [u8]) -> crate::util::ParseResult<'_, $packet_type> {
                $parse_body(data)
            }
            fn serialize_body(&self) -> Vec<u8> {
                $serialize_body(self)
            }
        }
        impl From<$packet_type> for crate::packet::GenericPacket {
            fn from(value: $packet_type) -> Self {
                crate::packet::GenericPacket::$packet_type(value)
            }
        }
        impl TryFrom<crate::packet::GenericPacket> for $packet_type {
            type Error = ();

            fn try_from(value: crate::packet::GenericPacket) -> Result<Self, Self::Error> {
                match value {
                    crate::packet::GenericPacket::$packet_type(packet) => Ok(packet),
                    _ => Err(()),
                }
            }
        }
    };
}
pub(crate) use packet;
