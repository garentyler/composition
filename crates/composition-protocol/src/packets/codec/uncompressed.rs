use crate::{
    packets::{Packet, PacketId},
    ClientState,
};
use composition_parsing::prelude::*;

pub fn decode(
    client_state: &ClientState,
    receive_serverbound_packets: bool,
    src: &mut bytes::BytesMut,
) -> Result<Option<Packet>, crate::Error> {
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
    let packet_body = Packet::parse_body(
        *client_state,
        packet_id,
        receive_serverbound_packets,
        &mut packet_data,
    );
    let packet_body = packet_body?;

    let bytes_read = src.remaining() - data.remaining();
    src.advance(bytes_read);

    Ok(Some(packet_body))
}

pub fn encode(packet: Packet, dst: &mut bytes::BytesMut) -> Result<(), crate::Error> {
    let mut output = vec![];
    let (packet_id, packet_body) = packet.serialize();
    output.extend(packet_id.serialize());
    output.extend(packet_body);
    let packet_len = VarInt::from(output.len() as i32).serialize();

    dst.reserve(packet_len.len() + output.len());
    dst.put(&packet_len[..]);
    dst.put(&output[..]);

    Ok(())
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

        match decode(&ClientState::Handshake, true, &mut data) {
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

        assert_eq!(output.len(), 0);
        encode(packet, &mut output).unwrap();
        assert_eq!(output.len(), 8);
        assert_eq!(
            &output[..],
            &[0x07, 0x00, 0x08, 0x01, 0x41, 0x12, 0x34, 0x01]
        );
    }
}
