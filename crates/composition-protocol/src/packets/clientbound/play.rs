use crate::{
    entities::{EntityPosition, EntityRotation, EntityVelocity},
    mctypes::{Chat, Difficulty, Position, Uuid, VarInt},
};
use bytes::Bytes;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP00SpawnEntity {
    pub id: VarInt,
    pub uuid: Uuid,
    pub kind: VarInt,
    pub position: EntityPosition,
    pub rotation: EntityRotation,
    pub head_yaw: u8,
    pub data: VarInt,
    pub velocity: EntityVelocity,
}
crate::packets::packet!(
    CP00SpawnEntity,
    0x00,
    crate::ClientState::Play,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CP00SpawnEntity> {
        Ok(CP00SpawnEntity {
            id: VarInt::parse(data)?,
            uuid: Uuid::parse(data)?,
            kind: VarInt::parse(data)?,
            position: EntityPosition::parse(data)?,
            rotation: EntityRotation::parse(data)?,
            head_yaw: u8::parse(data)?,
            data: VarInt::parse(data)?,
            velocity: EntityVelocity::parse(data)?,
        })
    },
    |packet: &CP00SpawnEntity| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.id.serialize());
        output.extend(packet.uuid.serialize());
        output.extend(packet.kind.serialize());
        output.extend(packet.position.serialize());
        output.extend(packet.rotation.serialize());
        output.extend(packet.head_yaw.serialize());
        output.extend(packet.data.serialize());
        output.extend(packet.velocity.serialize());
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP0BChangeDifficulty {
    pub difficulty: Difficulty,
    pub is_locked: bool,
}
crate::packets::packet!(
    CP0BChangeDifficulty,
    0x0b,
    crate::ClientState::Play,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CP0BChangeDifficulty> {
        Ok(CP0BChangeDifficulty {
            difficulty: Difficulty::parse(data)?,
            is_locked: bool::parse(data)?,
        })
    },
    |packet: &CP0BChangeDifficulty| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.difficulty.serialize());
        output.extend(packet.is_locked.serialize());
        output
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct CP17Disconnect {
    pub reason: Chat,
}
crate::packets::packet!(
    CP17Disconnect,
    0x17,
    crate::ClientState::Play,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CP17Disconnect> {
        Ok(CP17Disconnect {
            reason: Chat::parse(data)?,
        })
    },
    |packet: &CP17Disconnect| -> Vec<u8> { packet.reason.serialize() }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP1FKeepAlive {
    pub payload: i64,
}
crate::packets::packet!(
    CP1FKeepAlive,
    0x1f,
    crate::ClientState::Play,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CP1FKeepAlive> {
        Ok(CP1FKeepAlive {
            payload: i64::parse(data)?,
        })
    },
    |packet: &CP1FKeepAlive| -> Vec<u8> { packet.payload.serialize() }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP21WorldEvent {
    pub event: i32,
    pub location: Position,
    pub data: i32,
    pub disable_relative_volume: bool,
}
crate::packets::packet!(
    CP21WorldEvent,
    0x21,
    crate::ClientState::Play,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CP21WorldEvent> {
        Ok(CP21WorldEvent {
            event: i32::parse(data)?,
            location: Position::parse(data)?,
            data: i32::parse(data)?,
            disable_relative_volume: bool::parse(data)?,
        })
    },
    |packet: &CP21WorldEvent| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.event.serialize());
        output.extend(packet.location.serialize());
        output.extend(packet.data.serialize());
        output.extend(packet.disable_relative_volume.serialize());
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP50SetEntityVelocity {
    pub entity_id: VarInt,
    pub entity_velocity: EntityVelocity,
}
crate::packets::packet!(
    CP50SetEntityVelocity,
    0x50,
    crate::ClientState::Play,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CP50SetEntityVelocity> {
        Ok(CP50SetEntityVelocity {
            entity_id: VarInt::parse(data)?,
            entity_velocity: EntityVelocity::parse(data)?,
        })
    },
    |packet: &CP50SetEntityVelocity| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.entity_id.serialize());
        output.extend(packet.entity_velocity.serialize());
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP52SetExperience {
    pub experience_bar: f32,
    pub total_experience: VarInt,
    pub level: VarInt,
}
crate::packets::packet!(
    CP52SetExperience,
    0x52,
    crate::ClientState::Play,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CP52SetExperience> {
        Ok(CP52SetExperience {
            experience_bar: f32::parse(data)?,
            total_experience: VarInt::parse(data)?,
            level: VarInt::parse(data)?,
        })
    },
    |packet: &CP52SetExperience| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.experience_bar.serialize());
        output.extend(packet.total_experience.serialize());
        output.extend(packet.level.serialize());
        output
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct CP68EntityEffect {
    pub entity_id: VarInt,
    pub effect_id: VarInt,
    pub amplifier: i8,
    pub duration: VarInt,
    pub is_ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    pub has_factor_data: bool,
    // TODO: pub factor_codec: NBT
}
crate::packets::packet!(
    CP68EntityEffect,
    0x68,
    crate::ClientState::Play,
    false,
    |data: &mut Bytes| -> composition_parsing::Result<CP68EntityEffect> {
        let entity_id = VarInt::parse(data)?;
        let effect_id = VarInt::parse(data)?;
        let amplifier = i8::parse(data)?;
        let duration = VarInt::parse(data)?;
        let flags = u8::parse(data)?;
        let is_ambient = flags & 0x01 > 0;
        let show_particles = flags & 0x02 > 0;
        let show_icon = flags & 0x04 > 0;
        let has_factor_data = bool::parse(data)?;
        // TODO: factor_codec

        Ok(CP68EntityEffect {
            entity_id,
            effect_id,
            amplifier,
            duration,
            is_ambient,
            show_particles,
            show_icon,
            has_factor_data,
        })
    },
    |packet: &CP68EntityEffect| -> Vec<u8> {
        let mut output = vec![];
        output.extend(packet.entity_id.serialize());
        output.extend(packet.effect_id.serialize());
        output.extend(packet.amplifier.serialize());
        output.extend(packet.duration.serialize());
        let mut flags = 0x00u8;
        if packet.is_ambient {
            flags |= 0x01;
        }
        if packet.show_particles {
            flags |= 0x02;
        }
        if packet.show_icon {
            flags |= 0x04;
        }
        output.extend(flags.serialize());
        // TODO: factor_codec
        output
    }
);
