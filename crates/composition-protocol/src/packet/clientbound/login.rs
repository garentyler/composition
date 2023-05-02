use crate::{util::*, Chat, Uuid};

#[derive(Clone, Debug, PartialEq)]
pub struct CL00Disconnect {
    pub reason: Chat,
}
crate::packet::packet!(
    CL00Disconnect,
    0x00,
    crate::ClientState::Login,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CL00Disconnect> {
        let (data, reason) = parse_json(data)?;
        Ok((data, CL00Disconnect { reason }))
    },
    |packet: &CL00Disconnect| -> Vec<u8> { serialize_json(&packet.reason) }
);

#[derive(Clone, Debug, PartialEq)]
pub struct CL01EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}
crate::packet::packet!(
    CL01EncryptionRequest,
    0x01,
    crate::ClientState::Login,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CL01EncryptionRequest> {
        let (data, server_id) = parse_string(data)?;
        let (data, public_key_len) = parse_varint(data)?;
        let (data, public_key) = take_bytes(public_key_len as usize)(data)?;
        let (data, verify_token_len) = parse_varint(data)?;
        let (data, verify_token) = take_bytes(verify_token_len as usize)(data)?;

        Ok((
            data,
            CL01EncryptionRequest {
                server_id,
                public_key: public_key.to_vec(),
                verify_token: verify_token.to_vec(),
            },
        ))
    },
    |packet: &CL01EncryptionRequest| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_string(&packet.server_id));
        output.extend_from_slice(&serialize_varint(packet.public_key.len() as i32));
        output.extend_from_slice(&packet.public_key);
        output.extend_from_slice(&serialize_varint(packet.verify_token.len() as i32));
        output.extend_from_slice(&packet.verify_token);
        output
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct CL02LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<CL02LoginSuccessProperty>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CL02LoginSuccessProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
impl CL02LoginSuccessProperty {
    pub fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, name) = parse_string(data)?;
        let (data, value) = parse_string(data)?;
        let (data, is_signed) = take_bytes(1usize)(data)?;
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
crate::packet::packet!(
    CL02LoginSuccess,
    0x02,
    crate::ClientState::Login,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CL02LoginSuccess> {
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
    },
    |packet: &CL02LoginSuccess| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_uuid(&packet.uuid));
        output.extend_from_slice(&serialize_string(&packet.username));
        output.extend_from_slice(&serialize_varint(packet.properties.len() as i32));
        for property in &packet.properties {
            output.extend_from_slice(&property.serialize());
        }
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CL03SetCompression {
    pub threshold: i32,
}
crate::packet::packet!(
    CL03SetCompression,
    0x03,
    crate::ClientState::Login,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CL03SetCompression> {
        let (data, threshold) = parse_varint(data)?;
        Ok((data, CL03SetCompression { threshold }))
    },
    |packet: &CL03SetCompression| -> Vec<u8> { serialize_varint(packet.threshold) }
);

#[derive(Clone, Debug, PartialEq)]
pub struct CL04LoginPluginRequest {
    pub message_id: i32,
    pub channel: String,
    pub data: Vec<u8>,
}
crate::packet::packet!(
    CL04LoginPluginRequest,
    0x04,
    crate::ClientState::Login,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CL04LoginPluginRequest> {
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
    },
    |packet: &CL04LoginPluginRequest| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(packet.message_id));
        output.extend_from_slice(&serialize_string(&packet.channel));
        output.extend_from_slice(&packet.data);
        output
    }
);
