use super::error::Error;
use crate::protocol::{
    encryption::*,
    packets::{Packet, PacketDirection},
    parsing::Parsable,
    types::VarInt,
    ClientState,
};
use tokio_util::{
    bytes::{Buf, BytesMut},
    codec::{Decoder, Encoder},
};
use tracing::trace;

#[derive(Clone, Debug)]
pub struct PacketCodec {
    pub client_state: ClientState,
    pub packet_direction: PacketDirection,
    pub aes_cipher: Option<(Aes128Cfb8Encryptor, Aes128Cfb8Decryptor, usize)>,
}
impl PacketCodec {
    pub fn new(client_state: ClientState, packet_direction: PacketDirection) -> PacketCodec {
        PacketCodec {
            client_state,
            packet_direction,
            aes_cipher: None,
        }
    }
}
impl Default for PacketCodec {
    fn default() -> Self {
        PacketCodec {
            client_state: ClientState::Handshake,
            packet_direction: PacketDirection::Serverbound,
            aes_cipher: None,
        }
    }
}
impl Decoder for PacketCodec {
    type Item = Packet;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // Bytes from [0..encryption_start] are decrypted.
        // Bytes from [encryption_start..] are decrypted.
        if let Some((_, ref mut aes_decryptor, encryption_start)) = self.aes_cipher.as_mut() {
            // We have to do a bunch of stupid type fuckery to get CFB8 working
            // because for some reason decrypt() consumes self?!
            let encrypted_src = src.split_off(*encryption_start);
            // Convert BytesMut to Vec<GenericArray<u8, UInt<UTerm, B1>>>
            let mut encrypted_src = encrypted_src
                .into_iter()
                .map(|b| *GenericCFB8BlockArray::from_slice(&[b]))
                .collect::<Vec<_>>();
            // Decrypt the bytes in place.
            aes_decryptor.decrypt_blocks_mut(encrypted_src.as_mut_slice());
            // Convert Vec<GenericArray<u8, UInt<UTerm, B1>>> to Vec<u8>
            let encrypted_src = encrypted_src
                .into_iter()
                .flat_map(|b| b.to_vec())
                .collect::<Vec<u8>>();

            // Append the decrypted bytes back to the source and move the encryption start position.
            *encryption_start += encrypted_src.len();
            src.extend_from_slice(&encrypted_src);
        }

        match Packet::parse(self.client_state, self.packet_direction, src) {
            Ok((rest, packet)) => {
                let bytes_consumed = src.len() - rest.len();
                src.advance(bytes_consumed);
                if let Some((_, _, encryption_start)) = &mut self.aes_cipher {
                    // Adjust the encryption start position if we are using AES encryption.
                    *encryption_start -= bytes_consumed;
                }

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
            Err(nom::Err::Failure(_)) => Err(Error::Parsing),
        }
    }
}
impl Encoder<Packet> for PacketCodec {
    type Error = Error;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut body = vec![];
        let (packet_id, packet_body) = item.serialize();
        body.extend(packet_id.serialize().to_vec());
        body.extend(packet_body);
        // TODO: Packet compression on `body`.
        let packet_len = VarInt::from(body.len()).serialize();
        let mut out = Vec::with_capacity(packet_len.len() + body.len());
        out.extend(packet_len);
        out.extend(body);

        if let Some((ref mut aes_encryptor, _, _)) = &mut self.aes_cipher {
            // We have to do a bunch of stupid type fuckery to get CFB8 working
            // because for some reason decrypt() consumes self?!
            // Convert Vec<u8> to Vec<GenericArray<u8, UInt<UTerm, B1>>>
            let mut encrypted_out = out
                .into_iter()
                .map(|b| *GenericCFB8BlockArray::from_slice(&[b]))
                .collect::<Vec<_>>();
            // Decrypt the bytes in place.
            aes_encryptor.encrypt_blocks_mut(encrypted_out.as_mut_slice());
            // Convert Vec<GenericArray<u8, UInt<UTerm, B1>>> to Vec<u8>
            let encrypted_out = encrypted_out
                .into_iter()
                .flat_map(|b| b.to_vec())
                .collect::<Vec<u8>>();

            out = encrypted_out;
        }

        dst.extend(out);
        Ok(())
    }
}
