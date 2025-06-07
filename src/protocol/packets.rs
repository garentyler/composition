#![allow(dead_code)]

// Inspired by https://github.com/iceiix/stevenarella.

use tracing::trace;

/// Enum representation of a packet's direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PacketDirection {
    Serverbound,
    Clientbound,
}

#[macro_export]
macro_rules! packets {
    ($($state:ident $state_name:ident {
        $($dir:ident $dir_name:ident {
            $(
                $(#[$attr:meta])*
                packet $name:ident $id:literal {
                    $($(#[$fattr:meta])* field $field:ident: $field_type:ty,)*
                    $($(#[$rattr:meta])* rest $rest:ident,)?
                }
            )*
        })+
    })+) => {
        use $crate::protocol::{ClientState, parsing::{VarInt, Parsable, IResult}};

        #[derive(Debug, Clone, PartialEq)]
        pub enum Packet {
            $($($(
                $name($state::$dir::$name),
            )*)+)+
        }
        $($($(
            impl From<$state::$dir::$name> for Packet {
                fn from(value: $state::$dir::$name) -> Packet {
                    Packet::$name(value)
                }
            }
        )*)+)+
        impl Packet {
            fn parser(client_state: ClientState, direction: PacketDirection) -> impl Fn(&[u8]) -> IResult<&[u8], Self> {
                move |input: &[u8]| {
                    use nom::{combinator::verify, bytes::streaming::take};

                    if client_state == ClientState::Disconnected {
                        return nom::combinator::fail(input);
                    }
                    let (input, packet_len) = VarInt::parse_usize(input)?;
                    let (input, packet_body) = take(packet_len)(input)?;
                    let (packet_body, packet_id) = verify(VarInt::parse, |v| {
                        match client_state {
                            $(ClientState::$state_name => {
                                match direction {
                                    $(PacketDirection::$dir_name => {
                                        match **v {
                                            $($id => true,)*
                                            _ => false,
                                        }
                                    })*
                                }
                            })*
                            ClientState::Disconnected => false,
                        }
                    })(packet_body)?;
                    trace!("Parsing packet: {:?} {:?} {:02x} ({} bytes) {}",
                        direction, client_state, *packet_id, packet_body.len(),
                        packet_body.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("")
                    );
                    let (_, packet) = Packet::body_parser(client_state, direction, packet_id)(packet_body)?;
                    // trace!("Parsed packet: {:?}", packet);
                    Ok((input, packet))
                }
            }
            fn body_parser(client_state: ClientState, direction: PacketDirection, packet_id: VarInt) -> impl Fn(&[u8]) -> IResult<&[u8], Self> {
                move |input: &[u8]| {
                    match client_state {
                        $(ClientState::$state_name => {
                            match direction {
                                $(PacketDirection::$dir_name => {
                                    match *packet_id {
                                        $($id => {
                                            let (rest, inner) = $state::$dir::$name::parse(input)?;
                                            // The packet should have consumed all of the input specified by packet_len.
                                            nom::combinator::eof(rest)?;
                                            Ok((rest, Packet::$name(inner)))
                                        },)*
                                        // Invalid packet id.
                                        _ => Ok(nom::combinator::fail(input)?),
                                    }
                                })*
                            }
                        })*
                        // Invalid client state.
                        _ => Ok(nom::combinator::fail(input)?),
                    }
                }
            }
            pub fn parse(client_state: ClientState, direction: PacketDirection, input: &[u8]) -> IResult<&[u8], Self> {
                Packet::parser(client_state, direction)(input)
            }
            pub fn parse_as<T: TryFrom<Packet, Error = Packet>>(client_state: ClientState, direction: PacketDirection, input: &[u8]) -> IResult<&[u8], Result<T, Self>> {
                nom::combinator::map(Self::parser(client_state, direction), T::try_from)(input)
            }
            pub fn serialize(&self) -> (VarInt, Vec<u8>) {
                match &self {
                    $($($(
                        Packet::$name(inner) => (VarInt::from($id), inner.serialize()),
                    )*)*)*
                }
            }
            pub fn state_change(&self) -> Option<ClientState> {
                match self {
                    Packet::Handshake(handshake) => Some(handshake.next_state),
                    Packet::LoginAcknowledged(_) => Some(ClientState::Configuration),
                    Packet::AcknowledgeFinishConfiguration(_) => Some(ClientState::Play),
                    Packet::LoginDisconnect(_) => Some(ClientState::Disconnected),
                    Packet::ConfigurationDisconnect(_) => Some(ClientState::Disconnected),
                    Packet::PlayDisconnect(_) => Some(ClientState::Disconnected),
                    Packet::PingResponse(_) => Some(ClientState::Disconnected),
                    _ => None,
                }
            }
        }

        $(pub mod $state {
            $(pub mod $dir {
                #![allow(unused_imports)]

                use $crate::protocol::{ClientState, parsing::{VarInt, Parsable, IResult}, types::*};
                use super::super::Packet;

                $(
                    $(#[$attr])*
                    #[derive(Default, Debug, Clone, PartialEq)]
                    pub struct $name {
                        $($(#[$fattr])* pub $field: $field_type,)*
                        $($(#[$rattr])* pub $rest: Vec<u8>,)?
                    }
                    impl TryFrom<Packet> for $name {
                        type Error = Packet;
                        fn try_from(value: Packet) -> Result<Self, Self::Error> {
                            match value {
                                Packet::$name(inner) => Ok(inner),
                                _ => Err(value),
                            }
                        }
                    }
                    impl Parsable for $name {
                        fn parse(input: &[u8]) -> IResult<&[u8], Self> {
                            $(let (input, $field) = <$field_type>::parse(input)?;)*
                            $(let (input, $rest) = nom::combinator::rest(input)?;)?
                            Ok((input, $name {
                                $($field: $field,)*
                                $($rest: $rest.to_vec(),)?
                            }))
                        }
                        #[allow(unused_mut)]
                        fn serialize(&self) -> Vec<u8> {
                            let mut output = vec![];
                            $(output.extend(self.$field.serialize());)*
                            $(output.extend(&self.$rest);)?
                            output
                        }
                    }
                )*
            })+
        })+
    };
}

packets!(
    handshake Handshake {
        serverbound Serverbound {
            packet Handshake 0x00 {
                field protocol_version: VarInt,
                field host: String,
                field port: u16,
                field next_state: ClientState,
            }
        }
        clientbound Clientbound {}
    }
    status Status {
        serverbound Serverbound {
            packet StatusRequest 0x00 {}
            packet PingRequest 0x01 {
                field payload: i64,
            }
        }
        clientbound Clientbound {
            packet StatusResponse 0x00 {
                field response: Json,
            }
            packet PingResponse 0x01 {
                field payload: i64,
            }
        }
    }
    login Login {
        serverbound Serverbound {
            packet LoginStart 0x00 {
                field name: String,
                field uuid: Option<Uuid>,
            }
            packet EncryptionResponse 0x01 {
                field shared_secret: Vec<u8>,
                field verify_token: Vec<u8>,
            }
            packet LoginPluginResponse 0x02 {
                field message_id: VarInt,
                // TODO: Implement
                rest data,
            }
            packet LoginAcknowledged 0x03 {}
            packet LoginCookieResponse 0x04 {
                // TODO: Implement
                rest data,
            }
        }
        clientbound Clientbound {
            packet LoginDisconnect 0x00 {
                field reason: Chat,
            }
            packet EncryptionRequest 0x01 {
                field server_id: String,
                field public_key: Vec<u8>,
                field verify_token: Vec<u8>,
                field use_mojang_authentication: bool,
            }
            packet LoginSuccess 0x02 {
                field uuid: Uuid,
                field username: String,
                // TODO: Implement
                rest properties,
            }
            packet SetCompression 0x03 {
                field threshold: VarInt,
            }
            packet LoginPluginRequest 0x04 {
                field message_id: VarInt,
                field channel: String,
                // TODO: Implement
                rest data,
            }
            packet LoginCookieRequest 0x05 {
                // TODO: Implement
                rest data,
            }
        }
    }
    configuration Configuration {
        serverbound Serverbound {
            packet ConfigurationClientInformation 0x00 {
                field locale: String,
                field view_distance: i8,
                field chat_mode: VarInt,
                field chat_colors: bool,
                field displayed_skin_parts: u8,
                field main_hand: VarInt,
                field enable_text_filtering: bool,
                field allow_server_listing: bool,
                field particle_status: VarInt,
            }
            packet ConfigurationCookieResponse 0x01 {
                field key: String,
                field payload: Option<Vec<u8>>,
            }
            packet ConfigurationServerboundPluginMessage 0x02 {
                field channel: String,
                rest data,
            }
            packet AcknowledgeFinishConfiguration 0x03 {}
            packet ConfigurationServerboundKeepAlive 0x04 {
                field payload: i64,
            }
            packet ConfigurationPong 0x05 {
                field payload: i32,
            }
            packet ConfigurationResourcePackResponse 0x06 {
                field uuid: Uuid,
                field result: VarInt,
            }
            packet ServerboundKnownPacks 0x07 {
                // TODO: Implement
                rest data,
            }
        }
        clientbound Clientbound {
            packet ConfigurationCookieRequest 0x00 {
                field key: String,
            }
            packet ConfigurationClientboundPluginMessage 0x01 {
                field channel: String,
                rest data,
            }
            packet ConfigurationDisconnect 0x02 {
                field reason: Chat,
            }
            packet FinishConfiguration 0x03 {}
            packet ConfigurationClientboundKeepAlive 0x04 {
                field payload: i64,
            }
            packet ConfigurationPing 0x05 {
                field payload: i32,
            }
            packet ResetChat 0x06 {}
            packet RegistryData 0x07 {
                // TODO: Implement
                rest data,
            }
            packet ConfigurationRemoveResourcePack 0x08 {
                field uuid: Option<Uuid>,
            }
            packet ConfigurationAddResourcePack 0x09 {
                field uuid: Uuid,
                field url: String,
                field hash: String,
                field forced: bool,
                field prompt: Option<Chat>,
            }
            packet ConfigurationStoreCookie 0x0A {
                field key: String,
                field payload: Vec<u8>,
            }
            packet ConfigurationTransfer 0x0B {
                field host: String,
                field port: u16,
            }
            packet FeatureFlags 0x0C {
                field feature_flags: Vec<String>,
            }
            packet ConfigurationUpdateTags 0x0D {
                // TODO: Implement
                rest data,
            }
            packet ClientboundKnownPacks 0x0E {
                // TODO: Implement
                rest data,
            }
            packet ConfigurationCustomReportDetails 0x0F {
                // TODO: Implement
                rest data,
            }
            packet ConfigurationServerLinks 0x10 {
                // TODO: Implement
                rest data,
            }
        }
    }
    play Play {
        serverbound Serverbound {}
        clientbound Clientbound {
            packet PlayDisconnect 0x17 {
                field reason: Chat,
            }
        }
    }
);

#[cfg(test)]
mod tests {
    use super::{Packet, PacketDirection};
    use crate::protocol::{packets::handshake::serverbound::Handshake, types::VarInt, ClientState};

    fn get_handshake() -> (Handshake, &'static [u8]) {
        (
            Handshake {
                protocol_version: VarInt::from(767),
                host: String::from("localhost"),
                port: 25565,
                next_state: ClientState::Status,
            },
            &[
                // Packet length
                0x10, // Packet ID
                0x00, // protocol_version: VarInt
                0xff, 0x05, // host: String
                0x09, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x68, 0x6f, 0x73, 0x74, // port: u16
                0x63, 0xdd, // next_state: ClientState (VarInt)
                0x01,
            ],
        )
    }

    #[test]
    fn packet_parsing_works() {
        let (handshake, handshake_bytes) = get_handshake();

        let (rest, packet) = Packet::parse(
            ClientState::Handshake,
            PacketDirection::Serverbound,
            handshake_bytes,
        )
        .unwrap();
        assert_eq!(packet, Packet::Handshake(handshake));
        assert!(rest.is_empty());
    }
}
