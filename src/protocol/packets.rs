#![allow(dead_code)]

// Inspired by https://github.com/iceiix/stevenarella.

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
                    })(input)?;
                    let (input, packet_body) = take(packet_len)(packet_body)?;
                    let (_, packet) = Packet::body_parser(client_state, direction, packet_id)(packet_body)?;
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
                field successful: bool,
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
            }
            packet LoginSuccess 0x02 {
                field uuid: Uuid,
                field username: String,
                // TODO: Re-implement CL02LoginSuccessProperty
                rest properties,
            }
            packet SetCompression 0x03 {
                field threshold: VarInt,
            }
            packet LoginPluginRequest 0x04 {
                field message_id: VarInt,
                field channel: String,
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
