/// Packets that are heading to the client.
pub mod clientbound;
/// Packets that are heading to the server.
pub mod serverbound;

use crate::protocol::parsing::{Parsable, VarInt};

/// Alias for a `VarInt`.
pub type PacketId = VarInt;

pub trait Packet:
    std::fmt::Debug + Clone + TryFrom<GenericPacket> + Into<GenericPacket> + Parsable
{
    const ID: i32;
    const CLIENT_STATE: crate::protocol::ClientState;
    const IS_SERVERBOUND: bool;
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
                client_state: crate::protocol::ClientState,
                is_serverbound: bool,
                data: &'data [u8]
            ) -> crate::protocol::parsing::IResult<&'data [u8], Self> {
                use crate::protocol::parsing::Parsable;
                tracing::trace!(
                    "GenericPacket::parse_uncompressed: {:?} {} {:?}",
                    client_state,
                    is_serverbound,
                    data
                );
                let (data, packet_length) = crate::protocol::types::VarInt::parse(data)?;
                let (data, packet_data) = nom::bytes::streaming::take(*packet_length as usize)(data)?;

                let (packet_data, packet_id) = PacketId::parse(packet_data)?;
                let (_packet_data, packet_body) =
                    Self::parse_body(client_state, packet_id, is_serverbound, packet_data)?;

                // if !packet_data.is_empty() {
                //     println!("Packet data not empty after parsing!");
                // }

                Ok((data, packet_body))
            }

            #[tracing::instrument]
            pub fn parse_body<'data>(
                client_state: crate::protocol::ClientState,
                packet_id: crate::protocol::packets::PacketId,
                is_serverbound: bool,
                data: &'data [u8],
            ) -> crate::protocol::parsing::IResult<&'data [u8], Self> {
                use crate::protocol::parsing::Parsable;
                tracing::trace!(
                    "GenericPacket::parse_body: {:?} {} {}",
                    client_state,
                    packet_id,
                    is_serverbound
                );
                match (client_state, *packet_id, is_serverbound) {
                    $(
                        ($packet_type::CLIENT_STATE, $packet_type::ID, $packet_type::IS_SERVERBOUND) => $packet_type::parse(data).map(|(data, packet)| (data, Into::<GenericPacket>::into(packet))),
                    )*
                    _ => Ok((data, Self::UnimplementedPacket(UnimplementedPacket(packet_id)))),
                }
            }

            #[tracing::instrument]
            pub fn serialize(&self) -> (crate::protocol::packets::PacketId, Vec<u8>) {
                use crate::protocol::parsing::Parsable;
                tracing::trace!("GenericPacket::serialize: {:?}", self);
                match self {
                    $(
                        Self::$packet_type(packet) => (PacketId::from($packet_type::ID), packet.serialize()),
                    )*
                }
            }
        }
    };
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnimplementedPacket(VarInt);
packet!(
    UnimplementedPacket,
    0x00,
    crate::protocol::ClientState::Disconnected,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], UnimplementedPacket> {
        Ok((data, UnimplementedPacket(0i32.into())))
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
        impl crate::protocol::packets::Packet for $packet_type {
            const ID: i32 = $id;
            const CLIENT_STATE: crate::protocol::ClientState = $client_state;
            const IS_SERVERBOUND: bool = $serverbound;
        }
        impl crate::protocol::parsing::Parsable for $packet_type {
            #[tracing::instrument]
            fn parse<'data>(
                data: &'data [u8],
            ) -> crate::protocol::parsing::IResult<&'data [u8], Self> {
                $parse_body(data)
            }
            #[tracing::instrument]
            fn serialize(&self) -> Vec<u8> {
                $serialize_body(self)
            }
        }
        impl From<$packet_type> for crate::protocol::packets::GenericPacket {
            fn from(value: $packet_type) -> Self {
                crate::protocol::packets::GenericPacket::$packet_type(value)
            }
        }
        impl TryFrom<crate::protocol::packets::GenericPacket> for $packet_type {
            type Error = ();

            fn try_from(
                value: crate::protocol::packets::GenericPacket,
            ) -> Result<Self, Self::Error> {
                match value {
                    crate::protocol::packets::GenericPacket::$packet_type(packet) => Ok(packet),
                    _ => Err(()),
                }
            }
        }
    };
}
pub(crate) use packet;
