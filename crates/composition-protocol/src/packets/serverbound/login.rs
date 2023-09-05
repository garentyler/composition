use crate::mctypes::{Uuid, VarInt};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq)]
pub struct SL00LoginStart {
    pub name: String,
    pub uuid: Option<Uuid>,
}
crate::packets::packet!(
    SL00LoginStart,
    0x00,
    crate::ClientState::Login,
    true,
    |data: &mut Bytes| -> composition_parsing::Result<SL00LoginStart> {
        Ok(SL00LoginStart {
            name: String::parse(data)?,
            uuid: Uuid::parse_optional(data)?,
        })
    },
    |packet: &SL00LoginStart| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.name.serialize());
        output.extend(packet.uuid.serialize());
        output
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct SL01EncryptionResponse {
    pub shared_secret: Bytes,
    pub verify_token: Bytes,
}
crate::packets::packet!(
    SL01EncryptionResponse,
    0x01,
    crate::ClientState::Login,
    true,
    |data: &mut Bytes| -> composition_parsing::Result<SL01EncryptionResponse> {
        let shared_secret = Bytes::from(u8::parse_vec(data)?);
        let verify_token = Bytes::from(u8::parse_vec(data)?);

        Ok(SL01EncryptionResponse {
            shared_secret,
            verify_token,
        })
    },
    |packet: &SL01EncryptionResponse| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.shared_secret.to_vec().serialize());
        output.extend(packet.verify_token.to_vec().serialize());
        output
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct SL02LoginPluginResponse {
    pub message_id: VarInt,
    pub successful: bool,
    pub data: Bytes,
}
crate::packets::packet!(
    SL02LoginPluginResponse,
    0x02,
    crate::ClientState::Login,
    true,
    |data: &mut Bytes| -> composition_parsing::Result<SL02LoginPluginResponse> {
        let message_id = VarInt::parse(data)?;
        let successful = bool::parse(data)?;
        let d = {
            if successful {
                // Consume the rest of the data.
                let remaining_bytes = data.remaining();
                let d = data.copy_to_bytes(remaining_bytes);
                data.advance(remaining_bytes);
                d
            } else {
                Bytes::new()
            }
        };

        Ok(SL02LoginPluginResponse {
            message_id,
            successful,
            data: d,
        })
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
