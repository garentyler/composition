use crate::protocol::types::{Chat, Json, Uuid, VarInt};
use crate::protocol::parsing::Parsable;

#[derive(Clone, Debug, PartialEq)]
pub struct CL00Disconnect {
    pub reason: Chat,
}
crate::protocol::packets::packet!(
    CL00Disconnect,
    0x00,
    crate::protocol::ClientState::Login,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CL00Disconnect> {
        let (data, reason) = Json::parse(data)?;
        Ok((data, CL00Disconnect { reason }))
    },
    |packet: &CL00Disconnect| -> Vec<u8> { packet.reason.serialize() }
);

#[derive(Clone, Debug, PartialEq)]
pub struct CL01EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}
crate::protocol::packets::packet!(
    CL01EncryptionRequest,
    0x01,
    crate::protocol::ClientState::Login,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CL01EncryptionRequest> {
        let (data, server_id) = String::parse(data)?;
        let (data, public_key) = u8::parse_vec(data)?;
        let (data, verify_token) = u8::parse_vec(data)?;

        Ok((data, CL01EncryptionRequest {
            server_id,
            public_key,
            verify_token,
        }))
    },
    |packet: &CL01EncryptionRequest| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.server_id.serialize());
        output.extend(packet.public_key.serialize());
        output.extend(packet.verify_token.serialize());
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
    #[tracing::instrument]
    fn parse(data: &[u8]) -> crate::protocol::parsing::IResult<&[u8], Self> {
        let (data, name) = String::parse(data)?;
        let (data, value) = String::parse(data)?;
        let (data, signature) = String::parse_optional(data)?;
        Ok((data, CL02LoginSuccessProperty {
            name,
            value,
            signature,
        }))
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
crate::protocol::packets::packet!(
    CL02LoginSuccess,
    0x02,
    crate::protocol::ClientState::Login,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CL02LoginSuccess> {
        let (data, uuid) = Uuid::parse(data)?;
        let (data, username) = String::parse(data)?;
        let (data, properties) = CL02LoginSuccessProperty::parse_vec(data)?;

        Ok((data, CL02LoginSuccess {
            uuid,
            username,
            properties,
        }))
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
crate::protocol::packets::packet!(
    CL03SetCompression,
    0x03,
    crate::protocol::ClientState::Login,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CL03SetCompression> {
        let (data, threshold) = VarInt::parse(data)?;
        Ok((data, CL03SetCompression { threshold }))
    },
    |packet: &CL03SetCompression| -> Vec<u8> { packet.threshold.serialize() }
);

#[derive(Clone, Debug, PartialEq)]
pub struct CL04LoginPluginRequest {
    pub message_id: VarInt,
    pub channel: String,
    pub data: Vec<u8>,
}
crate::protocol::packets::packet!(
    CL04LoginPluginRequest,
    0x04,
    crate::protocol::ClientState::Login,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CL04LoginPluginRequest> {
        let (data, message_id) = VarInt::parse(data)?;
        let (data, channel) = String::parse(data)?;
        Ok((data, CL04LoginPluginRequest {
            message_id,
            channel,
            data: data.to_vec(),
        }))
    },
    |packet: &CL04LoginPluginRequest| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.message_id.serialize());
        output.extend(packet.channel.serialize());
        output.extend(&packet.data);
        output
    }
);
