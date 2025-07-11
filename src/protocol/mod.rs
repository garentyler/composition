/// Implementation of Minecraft's blocks.
pub mod blocks;
/// Protocol encryption.
pub mod encryption;
/// Implementation of Minecraft's entities.
pub mod entities;
/// When using the protocol encounters errors.
pub mod error;
/// Implementation of Minecraft's items and inventories.
pub mod inventory;
/// Network packets.
///
/// Packet names are as found on [wiki.vg](https://wiki.vg/Protocol)
/// in PascalCase, with some exceptions for uniqueness.
pub mod packets;
/// Useful shared parsing functions.
pub mod parsing;
/// Useful types for representing the Minecraft protocol.
pub mod types;

pub use error::{Error, Result};
use types::VarInt;

/// Enum representation of the connection's current state.
///
/// Parsing packets requires knowing which state the connection is in.
/// [Relevant wiki.vg page](https://wiki.vg/How_to_Write_a_Server#FSM_example_of_handling_new_TCP_connections)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ClientState {
    /// The connection is freshly established.
    ///
    /// The only packet in this state is `SH00Handshake`.
    /// After this packet is sent, the connection immediately
    /// transitions to `Status` or `Login`.
    #[default]
    Handshake,
    /// The client is performing [server list ping](https://wiki.vg/Server_List_Ping).
    Status,
    /// The client is attempting to join the server.
    ///
    /// The `Login` state includes authentication, encryption, compression, and plugins.
    Login,
    /// The client is in the `Configuration` state.
    Configuration,
    /// The main connection state. The client has authenticated and is playing on the server.
    Play,
    /// The client has disconnected, and the connection struct should be removed. No packets should be sent or received.
    Disconnected,
}
impl parsing::Parsable for ClientState {
    fn parse(data: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized,
    {
        nom::combinator::map_res(VarInt::parse, |next_state: VarInt| match *next_state {
            1 => Ok(ClientState::Status),
            2 => Ok(ClientState::Login),
            _ => Err(()),
        })(data)
    }
    fn serialize(&self) -> Vec<u8> {
        let byte = match &self {
            ClientState::Status => 1,
            ClientState::Login => 2,
            _ => 0,
        };
        vec![byte]
    }
}
