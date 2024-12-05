/// Implementation of Minecraft's blocks.
pub mod blocks;
/// Implementation of Minecraft's entities.
pub mod entities;
/// When using the protocol encounters errors.
pub mod error;
/// Implementation of Minecraft's items and inventories.
pub mod inventory;
/// Useful types for representing the Minecraft protocol.
pub mod types;
/// Network packets.
///
/// The packet naming convention used is "DSIDName" where
/// 'D' is either 'S' for serverbound or 'C' for clientbound,
/// 'S' is the current connection state (**H**andshake, **S**tatus, **L**ogin, or **P**lay),
/// "ID" is the packet id in uppercase hexadecimal (ex. 1B, 05, 3A),
/// and "Name" is the packet's name as found on [wiki.vg](https://wiki.vg/Protocol) in PascalCase.
/// Examples include "SH00Handshake", "CP00SpawnEntity", and "SP11KeepAlive".
pub mod packets;
/// Useful shared parsing functions.
pub mod parsing;

pub use error::{Error, Result};

/// Enum representation of the connection's current state.
///
/// Parsing packets requires knowing which state the connection is in.
/// [Relevant wiki.vg page](https://wiki.vg/How_to_Write_a_Server#FSM_example_of_handling_new_TCP_connections)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClientState {
    /// The connection is freshly established.
    ///
    /// The only packet in this state is `SH00Handshake`.
    /// After this packet is sent, the connection immediately
    /// transitions to `Status` or `Login`.
    Handshake,
    /// The client is performing [server list ping](https://wiki.vg/Server_List_Ping).
    Status,
    /// The client is attempting to join the server.
    ///
    /// The `Login` state includes authentication, encryption, compression, and plugins.
    Login,
    /// The main connection state. The client has authenticated and is playing on the server.
    Play,
    /// The client has disconnected, and the connection struct should be removed. No packets should be sent or received.
    Disconnected,
}
