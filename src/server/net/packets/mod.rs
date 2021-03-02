pub mod clientbound;
pub mod serverbound;

use crate::mctypes::{MCType, MCVarInt};
pub use clientbound::*;
pub use serverbound::*;
use std::convert::{Into, TryFrom};
use std::net::TcpStream;

pub trait Packet: Into<Vec<u8>> + TryFrom<Vec<u8>> {
    fn new() -> Self;
    fn read(_stream: &mut TcpStream) -> std::io::Result<Self>;
    fn write(&self, _stream: &mut TcpStream) -> std::io::Result<()>;
}

pub fn read_packet_header(t: &mut TcpStream) -> std::io::Result<(MCVarInt, MCVarInt)> {
    let length = MCVarInt::read(t)?;
    let id = MCVarInt::read(t)?;
    Ok((length, id))
}
