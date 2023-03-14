use crate::{
    packet::{GenericPacket, Packet, PacketId},
    util::{
        parse_string, parse_uuid, parse_varint, serialize_string, serialize_uuid, serialize_varint,
    },
    Uuid,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SL00LoginStart {
    name: String,
    uuid: Option<Uuid>,
}
impl Packet for SL00LoginStart {
    fn id() -> PacketId {
        0x00
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Login
    }
    fn serverbound() -> bool {
        true
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, name) = parse_string(data)?;
        let (data, has_uuid) = nom::bytes::streaming::take(1usize)(data)?;
        if has_uuid == [0x01] {
            let (data, uuid) = parse_uuid(data)?;
            Ok((
                data,
                SL00LoginStart {
                    name,
                    uuid: Some(uuid),
                },
            ))
        } else {
            Ok((data, SL00LoginStart { name, uuid: None }))
        }
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_string(&self.name));
        match self.uuid {
            Some(uuid) => {
                output.push(0x01);
                output.extend_from_slice(&serialize_uuid(&uuid));
            }
            None => output.push(0x00),
        }
        output
    }
}
impl From<SL00LoginStart> for GenericPacket {
    fn from(value: SL00LoginStart) -> Self {
        GenericPacket::SL00LoginStart(value)
    }
}
impl TryFrom<GenericPacket> for SL00LoginStart {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::SL00LoginStart(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SL01EncryptionResponse {
    shared_secret: Vec<u8>,
    verify_token: Vec<u8>,
}
impl Packet for SL01EncryptionResponse {
    fn id() -> PacketId {
        0x01
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Login
    }
    fn serverbound() -> bool {
        true
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, shared_secret_len) = parse_varint(data)?;
        let (data, shared_secret) = nom::bytes::streaming::take(shared_secret_len as usize)(data)?;
        let (data, verify_token_len) = parse_varint(data)?;
        let (data, verify_token) = nom::bytes::streaming::take(verify_token_len as usize)(data)?;

        Ok((
            data,
            SL01EncryptionResponse {
                shared_secret: shared_secret.to_vec(),
                verify_token: verify_token.to_vec(),
            },
        ))
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(self.shared_secret.len() as i32));
        output.extend_from_slice(&self.shared_secret);
        output.extend_from_slice(&serialize_varint(self.verify_token.len() as i32));
        output.extend_from_slice(&self.verify_token);
        output
    }
}
impl From<SL01EncryptionResponse> for GenericPacket {
    fn from(value: SL01EncryptionResponse) -> Self {
        GenericPacket::SL01EncryptionResponse(value)
    }
}
impl TryFrom<GenericPacket> for SL01EncryptionResponse {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::SL01EncryptionResponse(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SL02LoginPluginResponse {
    message_id: i32,
    successful: bool,
    data: Vec<u8>,
}
impl Packet for SL02LoginPluginResponse {
    fn id() -> PacketId {
        0x02
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Login
    }
    fn serverbound() -> bool {
        true
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, message_id) = parse_varint(data)?;
        let (data, successful) = nom::bytes::streaming::take(1usize)(data)?;
        let successful = successful == [0x01];
        Ok((
            data,
            SL02LoginPluginResponse {
                message_id,
                successful,
                data: match successful {
                    true => data.to_vec(),
                    false => vec![],
                },
            },
        ))
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(self.message_id));
        if self.successful {
            output.push(0x01);
            output.extend_from_slice(&self.data);
        } else {
            output.push(0x00);
        }
        output
    }
}
impl From<SL02LoginPluginResponse> for GenericPacket {
    fn from(value: SL02LoginPluginResponse) -> Self {
        GenericPacket::SL02LoginPluginResponse(value)
    }
}
impl TryFrom<GenericPacket> for SL02LoginPluginResponse {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::SL02LoginPluginResponse(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}
