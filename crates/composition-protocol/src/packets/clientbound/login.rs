use crate::mctypes::{Chat, Uuid, VarInt};
use bytes::Bytes;
use composition_parsing::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CL00Disconnect {
    pub reason: Chat,
}
crate::packets::packet!(
    CL00Disconnect,
    0x00,
    crate::ClientState::Login,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CL00Disconnect> {
        Ok(CL00Disconnect {
            reason: Chat::parse(data)?,
        })
    },
    |packet: &CL00Disconnect| -> Vec<u8> { packet.reason.serialize() }
);

#[derive(Clone, Debug, PartialEq)]
pub struct CL01EncryptionRequest {
    pub server_id: String,
    pub public_key: Bytes,
    pub verify_token: Bytes,
}

crate::packets::packet!(
    CL01EncryptionRequest,
    0x01,
    crate::ClientState::Login,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CL01EncryptionRequest> {
        Ok(CL01EncryptionRequest {
            server_id: String::parse(data)?,
            public_key: Bytes::from(u8::parse_vec(data)?),
            verify_token: Bytes::from(u8::parse_vec(data)?),
        })
    },
    |packet: &CL01EncryptionRequest| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.server_id.serialize());
        output.extend(packet.public_key.to_vec().serialize());
        output.extend(packet.verify_token.to_vec().serialize());
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
impl Parsable for CL02LoginSuccessProperty {
    fn check(mut data: Bytes) -> composition_parsing::Result<()> {
        Self::parse(&mut data).map(|_| ())
    }
    fn parse(data: &mut Bytes) -> composition_parsing::Result<Self> {
        Ok(CL02LoginSuccessProperty {
            name: String::parse(data)?,
            value: String::parse(data)?,
            signature: String::parse_optional(data)?,
        })
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend(self.name.serialize());
        output.extend(self.value.serialize());
        output.extend(self.signature.serialize());
        output
    }
}
crate::packets::packet!(
    CL02LoginSuccess,
    0x02,
    crate::ClientState::Login,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CL02LoginSuccess> {
        Ok(CL02LoginSuccess {
            uuid: Uuid::parse(data)?,
            username: String::parse(data)?,
            properties: CL02LoginSuccessProperty::parse_vec(data)?,
        })
    },
    |packet: &CL02LoginSuccess| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.uuid.serialize());
        output.extend(packet.username.serialize());
        output.extend(packet.properties.serialize());
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CL03SetCompression {
    pub threshold: VarInt,
}
crate::packets::packet!(
    CL03SetCompression,
    0x03,
    crate::ClientState::Login,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CL03SetCompression> {
        Ok(CL03SetCompression {
            threshold: VarInt::parse(data)?,
        })
    },
    |packet: &CL03SetCompression| -> Vec<u8> { packet.threshold.serialize() }
);

#[derive(Clone, Debug, PartialEq)]
pub struct CL04LoginPluginRequest {
    pub message_id: VarInt,
    pub channel: String,
    pub data: Bytes,
}
crate::packets::packet!(
    CL04LoginPluginRequest,
    0x04,
    crate::ClientState::Login,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CL04LoginPluginRequest> {
        let message_id = VarInt::parse(data)?;
        let channel = String::parse(data)?;
        // Consume the rest of the data.
        let remaining_bytes = data.remaining();
        let d = data.copy_to_bytes(remaining_bytes);
        data.advance(remaining_bytes);

        Ok(CL04LoginPluginRequest {
            message_id,
            channel,
            data: d,
        })
    },
    |packet: &CL04LoginPluginRequest| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.message_id.serialize());
        output.extend(packet.channel.serialize());
        output.extend(&packet.data);
        output
    }
);
