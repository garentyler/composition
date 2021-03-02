/// The packets that get sent to the client by the server.
pub mod clientbound;
/// The packets that get sent to the server by the client.
pub mod serverbound;

use crate::mctypes::MCVarInt;
pub use clientbound::*;
pub use serverbound::*;
use std::net::TcpStream;

/// A helper function to read the packet header.
pub async fn read_packet_header(t: &mut TcpStream) -> std::io::Result<(MCVarInt, MCVarInt)> {
    let length = MCVarInt::read(t).await?;
    let id = MCVarInt::read(t).await?;
    Ok((length, id))
}
