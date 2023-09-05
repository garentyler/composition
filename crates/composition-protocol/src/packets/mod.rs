/// Packets that are heading to the client.
pub mod clientbound;
/// Framing I/O streams using the packets and protocol.
#[cfg(feature = "codec")]
pub mod codec;
/// Packets that are heading to the server.
pub mod serverbound;

use crate::mctypes::VarInt;
use bytes::Bytes;
use composition_parsing::prelude::*;

/// Alias for a `VarInt`.
pub type PacketId = VarInt;

pub trait PacketInfo: std::fmt::Debug + Clone + TryFrom<Packet> + Into<Packet> + Parsable {
    const ID: i32;
    const CLIENT_STATE: crate::ClientState;
    const IS_SERVERBOUND: bool;
}

macro_rules! generic_packet {
    ($($packet_type: ident),*) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum Packet {
            $(
                $packet_type($packet_type),
            )*
        }
        impl Packet {
            pub fn parse_body(
                client_state: crate::ClientState,
                packet_id: crate::packets::PacketId,
                is_serverbound: bool,
                data: &mut Bytes
            ) -> composition_parsing::Result<Self> {
                use composition_parsing::parsable::Parsable;
                match (client_state, *packet_id, is_serverbound) {
                    $(
                        ($packet_type::CLIENT_STATE, $packet_type::ID, $packet_type::IS_SERVERBOUND) => $packet_type::parse(data).map(|packet| Into::<Packet>::into(packet)),
                    )*
                    _ => Ok(Self::UnimplementedPacket(UnimplementedPacket(packet_id))),
                }
            }

            pub fn serialize(&self) -> (crate::packets::PacketId, Vec<u8>) {
                use composition_parsing::parsable::Parsable;
                tracing::trace!("Packet::serialize: {:?}", self);
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
    crate::ClientState::Disconnected,
    false,
    |_data: &mut Bytes| -> composition_parsing::Result<UnimplementedPacket> {
        Ok(UnimplementedPacket(0i32.into()))
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
        impl crate::packets::PacketInfo for $packet_type {
            const ID: i32 = $id;
            const CLIENT_STATE: crate::ClientState = $client_state;
            const IS_SERVERBOUND: bool = $serverbound;
        }
        impl composition_parsing::parsable::Parsable for $packet_type {
            fn check(mut data: bytes::Bytes) -> composition_parsing::Result<()> {
                Self::parse(&mut data).map(|_| ())
            }
            fn parse(data: &mut bytes::Bytes) -> composition_parsing::Result<Self> {
                $parse_body(data)
            }
            fn serialize(&self) -> Vec<u8> {
                $serialize_body(self)
            }
        }
        impl From<$packet_type> for crate::packets::Packet {
            fn from(value: $packet_type) -> Self {
                crate::packets::Packet::$packet_type(value)
            }
        }
        impl TryFrom<crate::packets::Packet> for $packet_type {
            type Error = ();

            fn try_from(value: crate::packets::Packet) -> Result<Self, Self::Error> {
                match value {
                    crate::packets::Packet::$packet_type(packet) => Ok(packet),
                    _ => Err(()),
                }
            }
        }
    };
}
pub(crate) use packet;
