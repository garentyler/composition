/// The packets that get sent to the client by the server.
pub mod clientbound;
/// The packets that get sent to the server by the client.
pub mod serverbound;

use crate::mctypes::{MCType, MCVarInt};
pub use clientbound::*;
pub use serverbound::*;
use std::convert::{Into, TryFrom};
use std::net::TcpStream;

/// All packets need to serialize/deserialize to/from `Vec<u8>`.
pub trait Packet: Into<Vec<u8>> + TryFrom<Vec<u8>> {
    fn new() -> Self;
    /// Read the packet body from the given `TcpStream`.
    fn read(_stream: &mut TcpStream) -> std::io::Result<Self>;
    /// Write the packet (body and header) to the given `TcpStream`.
    fn write(&self, _stream: &mut TcpStream) -> std::io::Result<()>;
}

/// A helper function to read the packet header.
pub fn read_packet_header(t: &mut TcpStream) -> std::io::Result<(MCVarInt, MCVarInt)> {
    let length = MCVarInt::read(t)?;
    let id = MCVarInt::read(t)?;
    Ok((length, id))
}
