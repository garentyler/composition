use crate::{
    packet::{GenericPacket, Packet, PacketId},
    util::{
        parse_json, parse_string, parse_uuid, parse_varint, serialize_json, serialize_string,
        serialize_uuid, serialize_varint,
    },
    Chat, Uuid,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CL00Disconnect {
    reason: Chat,
}
impl Packet for CL00Disconnect {
    fn id() -> PacketId {
        0x00
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Login
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, reason) = parse_json(data)?;
        Ok((data, CL00Disconnect { reason }))
    }
    fn serialize_body(&self) -> Vec<u8> {
        serialize_json(&self.reason)
    }
}
impl From<CL00Disconnect> for GenericPacket {
    fn from(value: CL00Disconnect) -> Self {
        GenericPacket::CL00Disconnect(value)
    }
}
impl TryFrom<GenericPacket> for CL00Disconnect {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CL00Disconnect(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CL01EncryptionRequest {
    server_id: String,
    public_key: Vec<u8>,
    verify_token: Vec<u8>,
}
impl Packet for CL01EncryptionRequest {
    fn id() -> PacketId {
        0x01
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Login
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, server_id) = parse_string(data)?;
        let (data, public_key_len) = parse_varint(data)?;
        let (data, public_key) = nom::bytes::streaming::take(public_key_len as usize)(data)?;
        let (data, verify_token_len) = parse_varint(data)?;
        let (data, verify_token) = nom::bytes::streaming::take(verify_token_len as usize)(data)?;

        Ok((
            data,
            CL01EncryptionRequest {
                server_id,
                public_key: public_key.to_vec(),
                verify_token: verify_token.to_vec(),
            },
        ))
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_string(&self.server_id));
        output.extend_from_slice(&serialize_varint(self.public_key.len() as i32));
        output.extend_from_slice(&self.public_key);
        output.extend_from_slice(&serialize_varint(self.verify_token.len() as i32));
        output.extend_from_slice(&self.verify_token);
        output
    }
}
impl From<CL01EncryptionRequest> for GenericPacket {
    fn from(value: CL01EncryptionRequest) -> Self {
        GenericPacket::CL01EncryptionRequest(value)
    }
}
impl TryFrom<GenericPacket> for CL01EncryptionRequest {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CL01EncryptionRequest(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CL02LoginSuccess {
    uuid: Uuid,
    username: String,
    properties: Vec<CL02LoginSuccessProperty>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CL02LoginSuccessProperty {
    name: String,
    value: String,
    signature: Option<String>,
}
impl CL02LoginSuccessProperty {
    pub fn parse(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, name) = parse_string(data)?;
        let (data, value) = parse_string(data)?;
        let (data, is_signed) = nom::bytes::streaming::take(1usize)(data)?;
        if is_signed == [0x01] {
            let (data, signature) = parse_string(data)?;
            Ok((
                data,
                CL02LoginSuccessProperty {
                    name,
                    value,
                    signature: Some(signature),
                },
            ))
        } else {
            Ok((
                data,
                CL02LoginSuccessProperty {
                    name,
                    value,
                    signature: None,
                },
            ))
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_string(&self.name));
        output.extend_from_slice(&serialize_string(&self.value));
        match &self.signature {
            Some(signature) => {
                output.push(0x01);
                output.extend_from_slice(&serialize_string(signature));
            }
            None => output.push(0x00),
        }
        output
    }
}
impl Packet for CL02LoginSuccess {
    fn id() -> PacketId {
        0x02
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Login
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, uuid) = parse_uuid(data)?;
        let (data, username) = parse_string(data)?;
        let (mut data, properties_len) = parse_varint(data)?;
        let mut properties = Vec::with_capacity(properties_len as usize);
        for _ in 0..properties_len {
            let (d, property) = CL02LoginSuccessProperty::parse(data)?;
            data = d;
            properties.push(property);
        }

        Ok((
            data,
            CL02LoginSuccess {
                uuid,
                username,
                properties,
            },
        ))
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_uuid(&self.uuid));
        output.extend_from_slice(&serialize_string(&self.username));
        output.extend_from_slice(&serialize_varint(self.properties.len() as i32));
        for property in &self.properties {
            output.extend_from_slice(&property.serialize());
        }
        output
    }
}
impl From<CL02LoginSuccess> for GenericPacket {
    fn from(value: CL02LoginSuccess) -> Self {
        GenericPacket::CL02LoginSuccess(value)
    }
}
impl TryFrom<GenericPacket> for CL02LoginSuccess {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CL02LoginSuccess(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CL03SetCompression {
    threshold: i32,
}
impl Packet for CL03SetCompression {
    fn id() -> PacketId {
        0x03
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Login
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, threshold) = parse_varint(data)?;
        Ok((data, CL03SetCompression { threshold }))
    }
    fn serialize_body(&self) -> Vec<u8> {
        serialize_varint(self.threshold)
    }
}
impl From<CL03SetCompression> for GenericPacket {
    fn from(value: CL03SetCompression) -> Self {
        GenericPacket::CL03SetCompression(value)
    }
}
impl TryFrom<GenericPacket> for CL03SetCompression {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CL03SetCompression(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CL04LoginPluginRequest {
    message_id: i32,
    channel: String,
    data: Vec<u8>,
}
impl Packet for CL04LoginPluginRequest {
    fn id() -> PacketId {
        0x04
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Login
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, message_id) = parse_varint(data)?;
        let (data, channel) = parse_string(data)?;
        Ok((
            data,
            CL04LoginPluginRequest {
                message_id,
                channel,
                data: data.to_vec(),
            },
        ))
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(self.message_id));
        output.extend_from_slice(&serialize_string(&self.channel));
        output.extend_from_slice(&self.data);
        output
    }
}
impl From<CL04LoginPluginRequest> for GenericPacket {
    fn from(value: CL04LoginPluginRequest) -> Self {
        GenericPacket::CL04LoginPluginRequest(value)
    }
}
impl TryFrom<GenericPacket> for CL04LoginPluginRequest {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CL04LoginPluginRequest(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}
