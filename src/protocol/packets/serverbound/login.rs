use crate::protocol::types::{Uuid, VarInt};
use nom::bytes::streaming::take;

#[derive(Clone, Debug, PartialEq)]
pub struct SL00LoginStart {
    pub name: String,
    pub uuid: Option<Uuid>,
}
crate::protocol::packets::packet!(
    SL00LoginStart,
    0x00,
    crate::protocol::ClientState::Login,
    true,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], SL00LoginStart> {
        let (data, name) = String::parse(data)?;
        let (data, has_uuid) = bool::parse(data)?;
        if has_uuid {
            let (data, uuid) = Uuid::parse(data)?;
            Ok((data, SL00LoginStart {
                name,
                uuid: Some(uuid),
            }))
        } else {
            Ok((data, SL00LoginStart { name, uuid: None }))
        }
    },
    |packet: &SL00LoginStart| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.name.serialize());
        output.extend(packet.uuid.is_some().serialize());
        if let Some(uuid) = packet.uuid {
            output.extend(uuid.serialize());
        }
        output
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct SL01EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}
crate::protocol::packets::packet!(
    SL01EncryptionResponse,
    0x01,
    crate::protocol::ClientState::Login,
    true,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], SL01EncryptionResponse> {
        let (data, shared_secret_len) = VarInt::parse(data)?;
        let (data, shared_secret) = take(*shared_secret_len as usize)(data)?;
        let (data, verify_token_len) = VarInt::parse(data)?;
        let (data, verify_token) = take(*verify_token_len as usize)(data)?;

        Ok((data, SL01EncryptionResponse {
            shared_secret: shared_secret.to_vec(),
            verify_token: verify_token.to_vec(),
        }))
    },
    |packet: &SL01EncryptionResponse| -> Vec<u8> {
        let mut output = vec![];
        output.extend(VarInt::from(packet.shared_secret.len() as i32).serialize());
        output.extend(&packet.shared_secret);
        output.extend(VarInt::from(packet.verify_token.len() as i32).serialize());
        output.extend(&packet.verify_token);
        output
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct SL02LoginPluginResponse {
    pub message_id: VarInt,
    pub successful: bool,
    pub data: Vec<u8>,
}
crate::protocol::packets::packet!(
    SL02LoginPluginResponse,
    0x02,
    crate::protocol::ClientState::Login,
    true,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], SL02LoginPluginResponse> {
        let (data, message_id) = VarInt::parse(data)?;
        let (data, successful) = bool::parse(data)?;
        if successful {
            Ok((&[], SL02LoginPluginResponse {
                message_id,
                successful,
                data: data.to_vec(),
            }))
        } else {
            Ok((data, SL02LoginPluginResponse {
                message_id,
                successful,
                data: vec![],
            }))
        }
    },
    |packet: &SL02LoginPluginResponse| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.message_id.serialize());
        output.extend(packet.successful.serialize());
        if packet.successful {
            output.extend(&packet.data);
        }
        output
    }
);
