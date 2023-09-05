use crate::{
    packets::{Packet, PacketId},
    ClientState,
};
use composition_parsing::prelude::*;
use std::sync::{Arc, RwLock};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Clone, Debug)]
pub struct UncompressedPacketCodec {
    client_state: Arc<RwLock<ClientState>>,
    receive_serverbound_packets: bool,
}
impl UncompressedPacketCodec {
    pub fn new_client(client_state: Arc<RwLock<ClientState>>) -> Self {
        UncompressedPacketCodec {
            client_state,
            receive_serverbound_packets: false,
        }
    }
    pub fn new_server(client_state: Arc<RwLock<ClientState>>) -> Self {
        UncompressedPacketCodec {
            client_state,
            receive_serverbound_packets: true,
        }
    }
}
impl Decoder for UncompressedPacketCodec {
    type Item = Packet;
    type Error = crate::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // We don't want to mutate the original data.
        let mut data = src.clone().freeze();

        let packet_length = match VarInt::parse(&mut data) {
            Ok(len) => *len as usize,
            Err(composition_parsing::Error::Eof) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        if src.remaining() < packet_length {
            return Ok(None);
        }

        let mut packet_data = data.split_to(packet_length);
        let packet_id = PacketId::parse(&mut packet_data)?;

        // server rx serverbound, server tx clientbound
        // client rx clientbound, client tx serverbound
        let packet_body = {
            let client_state = self.client_state.read().unwrap();
            Packet::parse_body(
                *client_state,
                packet_id,
                self.receive_serverbound_packets,
                &mut packet_data,
            )
        };
        let packet_body = packet_body?;

        let bytes_read = src.remaining() - data.remaining();
        src.advance(bytes_read);

        // State machine logic.
        {
            let mut client_state = self.client_state.write().unwrap();

            match &packet_body {
                Packet::SH00Handshake(handshake) => {
                    *client_state = handshake.next_state;
                }
                Packet::CL02LoginSuccess(_) => {
                    *client_state = ClientState::Play;
                }
                Packet::CS01PingResponse(_)
                | Packet::CL00Disconnect(_)
                | Packet::CP17Disconnect(_) => {
                    *client_state = ClientState::Disconnected;
                }
                _ => {}
            }
        }

        Ok(Some(packet_body))
    }
}
impl Encoder<Packet> for UncompressedPacketCodec {
    type Error = crate::Error;

    fn encode(&mut self, item: Packet, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let mut output = vec![];
        let (packet_id, packet_body) = item.serialize();
        output.extend(packet_id.serialize());
        output.extend(packet_body);
        let packet_len = VarInt::from(output.len() as i32).serialize();

        dst.reserve(packet_len.len() + output.len());
        dst.put(&packet_len[..]);
        dst.put(&output[..]);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        packets::{serverbound::SH00Handshake, Packet},
        ClientState,
    };
    use bytes::BytesMut;

    #[test]
    fn handshake_decode() {
        let mut data = BytesMut::new();
        data.extend_from_slice(&[0x07, 0x00, 0x08, 0x01, 0x41, 0x12, 0x34, 0x01, 0xff]);

        let mut codec =
            UncompressedPacketCodec::new_server(Arc::new(RwLock::new(ClientState::Handshake)));

        match codec.decode(&mut data) {
            Ok(Some(Packet::SH00Handshake(handshake))) => {
                assert_eq!(
                    handshake,
                    SH00Handshake {
                        protocol_version: VarInt::from(8),
                        server_address: "A".to_string(),
                        server_port: 0x1234,
                        next_state: ClientState::Status,
                    }
                );

                // Check that we only consumed the right amount of data.
                assert!(data.get_u8() == 0xff);
            }
            Ok(Some(_)) => panic!("Codec returned the wrong packet"),
            Ok(None) => panic!("Codec incorrectly reached EOF"),
            Err(e) => panic!("Codec returned an error: {e}"),
        }
    }
    #[test]
    fn handshake_encode() {
        let mut output = BytesMut::new();
        let packet: Packet = SH00Handshake {
            protocol_version: VarInt::from(8),
            server_address: "A".to_string(),
            server_port: 0x1234,
            next_state: ClientState::Status,
        }
        .into();

        let mut codec =
            UncompressedPacketCodec::new_server(Arc::new(RwLock::new(ClientState::Handshake)));

        assert_eq!(output.len(), 0);
        codec.encode(packet, &mut output).unwrap();
        assert_eq!(output.len(), 8);
        assert_eq!(
            &output[..],
            &[0x07, 0x00, 0x08, 0x01, 0x41, 0x12, 0x34, 0x01]
        );
    }
}
