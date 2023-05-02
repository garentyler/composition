use crate::{util::*, Uuid};

#[derive(Clone, Debug, PartialEq)]
pub struct SL00LoginStart {
    pub name: String,
    pub uuid: Option<Uuid>,
}
crate::packet::packet!(
    SL00LoginStart,
    0x00,
    crate::ClientState::Login,
    true,
    |data: &'data [u8]| -> ParseResult<'data, SL00LoginStart> {
        let (data, name) = parse_string(data)?;
        let (data, has_uuid) = take_bytes(1usize)(data)?;
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
    },
    |packet: &SL00LoginStart| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_string(&packet.name));
        match packet.uuid {
            Some(uuid) => {
                output.push(0x01);
                output.extend_from_slice(&serialize_uuid(&uuid));
            }
            None => output.push(0x00),
        }
        output
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct SL01EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}
crate::packet::packet!(
    SL01EncryptionResponse,
    0x01,
    crate::ClientState::Login,
    true,
    |data: &'data [u8]| -> ParseResult<'data, SL01EncryptionResponse> {
        let (data, shared_secret_len) = parse_varint(data)?;
        let (data, shared_secret) = take_bytes(shared_secret_len as usize)(data)?;
        let (data, verify_token_len) = parse_varint(data)?;
        let (data, verify_token) = take_bytes(verify_token_len as usize)(data)?;

        Ok((
            data,
            SL01EncryptionResponse {
                shared_secret: shared_secret.to_vec(),
                verify_token: verify_token.to_vec(),
            },
        ))
    },
    |packet: &SL01EncryptionResponse| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(packet.shared_secret.len() as i32));
        output.extend_from_slice(&packet.shared_secret);
        output.extend_from_slice(&serialize_varint(packet.verify_token.len() as i32));
        output.extend_from_slice(&packet.verify_token);
        output
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct SL02LoginPluginResponse {
    pub message_id: i32,
    pub successful: bool,
    pub data: Vec<u8>,
}
crate::packet::packet!(
    SL02LoginPluginResponse,
    0x02,
    crate::ClientState::Login,
    true,
    |data: &'data [u8]| -> ParseResult<'data, SL02LoginPluginResponse> {
        let (data, message_id) = parse_varint(data)?;
        let (data, successful) = take_bytes(1usize)(data)?;
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
    },
    |packet: &SL02LoginPluginResponse| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(packet.message_id));
        if packet.successful {
            output.push(0x01);
            output.extend_from_slice(&packet.data);
        } else {
            output.push(0x00);
        }
        output
    }
);
