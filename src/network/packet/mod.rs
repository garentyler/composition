// network/packet/mod.rs
// authors: Garen Tyler
// description:
//   This module contains the packet structs.

#[derive(Debug)]
pub struct Packet {
    kind: PacketType,
}
#[derive(PartialEq, Debug)]
pub enum PacketType {
    Handshake,
}
