use crate::protocol::{
    packets::{Packet, PacketDirection},
    parsing::Parsable,
    types::VarInt,
    ClientState,
};
use tokio_util::{
    bytes::{Buf, BytesMut},
    codec::{Decoder, Encoder},
};
use super::error::Error;
use tracing::trace;

#[derive(Clone, Copy, Debug)]
pub struct PacketCodec {
    pub client_state: ClientState,
    pub packet_direction: PacketDirection,
}
impl PacketCodec {
    pub fn new(client_state: ClientState, packet_direction: PacketDirection) -> PacketCodec {
        PacketCodec {
            client_state,
            packet_direction,
        }
    }
}
impl Default for PacketCodec {
    fn default() -> Self {
        PacketCodec {
            client_state: ClientState::Handshake,
            packet_direction: PacketDirection::Serverbound,
        }
    }
}
impl Decoder for PacketCodec {
    type Item = Packet;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match Packet::parse(self.client_state, self.packet_direction, src) {
            Ok((rest, packet)) => {
                let bytes_consumed = src.len() - rest.len();
                src.advance(bytes_consumed);

                if let Some(next_state) = packet.state_change() {
                    self.client_state = next_state;
                }

                Ok(Some(packet))
            }
            Err(nom::Err::Incomplete(_)) => {
                // Try to read the packet length.
                match VarInt::parse_usize(src) {
                    Ok((_, packet_length)) => {
                        src.reserve(packet_length + 64);
                        Ok(None)
                    }
                    Err(nom::Err::Incomplete(_)) => {
                        src.reserve(5);
                        Ok(None)
                    }
                    Err(_) => Err(Error::Parsing),
                }
            }
            Err(nom::Err::Error(e)) => {
                trace!("parsing error: {:02X?}", e.input);
                Err(Error::Parsing)
            }
            Err(nom::Err::Failure(_)) => {
                Err(Error::Parsing)
            }
        }
    }
}
impl Encoder<Packet> for PacketCodec {
    type Error = Error;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut out = vec![];
        let (packet_id, packet_body) = item.serialize();
        out.extend(packet_id.serialize().to_vec());
        out.extend(packet_body);
        let packet_len = VarInt::from(out.len());
        dst.extend(packet_len.serialize());
        dst.extend(out);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn packet_decoder_works() {
        unimplemented!()
    }
    #[test]
    fn packet_encoder_works() {
        unimplemented!()
    }
}
