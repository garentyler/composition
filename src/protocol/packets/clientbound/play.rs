use crate::protocol::{
    entities::{EntityPosition, EntityRotation, EntityVelocity},
    types::{Chat, Difficulty, Position, Uuid, VarInt},
};

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
crate::protocol::packets::packet!(
    CP00SpawnEntity,
    0x00,
    crate::protocol::ClientState::Play,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CP00SpawnEntity> {
        let (data, id) = VarInt::parse(data)?;
        let (data, uuid) = Uuid::parse(data)?;
        let (data, kind) = VarInt::parse(data)?;
        let (data, position) = EntityPosition::parse(data)?;
        let (data, rotation) = EntityRotation::parse(data)?;
        let (data, head_yaw) = u8::parse(data)?;
        let (data, d) = VarInt::parse(data)?;
        let (data, velocity) = EntityVelocity::parse(data)?;

        Ok((
            data,
            CP00SpawnEntity {
                id,
                uuid,
                kind,
                position,
                rotation,
                head_yaw,
                data: d,
                velocity,
            },
        ))
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
crate::protocol::packets::packet!(
    CP0BChangeDifficulty,
    0x0b,
    crate::protocol::ClientState::Play,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CP0BChangeDifficulty> {
        let (data, difficulty) = Difficulty::parse(data)?;
        let (data, is_locked) = bool::parse(data)?;
        Ok((
            data,
            CP0BChangeDifficulty {
                difficulty,
                is_locked,
            },
        ))
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
crate::protocol::packets::packet!(
    CP17Disconnect,
    0x17,
    crate::protocol::ClientState::Play,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CP17Disconnect> {
        let (data, reason) = Chat::parse(data)?;
        Ok((data, CP17Disconnect { reason }))
    },
    |packet: &CP17Disconnect| -> Vec<u8> { packet.reason.serialize() }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP1FKeepAlive {
    pub payload: i64,
}
crate::protocol::packets::packet!(
    CP1FKeepAlive,
    0x1f,
    crate::protocol::ClientState::Play,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CP1FKeepAlive> {
        let (data, payload) = i64::parse(data)?;
        Ok((data, CP1FKeepAlive { payload }))
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
crate::protocol::packets::packet!(
    CP21WorldEvent,
    0x21,
    crate::protocol::ClientState::Play,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CP21WorldEvent> {
        let (data, event) = i32::parse(data)?;
        let (data, location) = Position::parse(data)?;
        let (data, d) = i32::parse(data)?;
        let (data, disable_relative_volume) = bool::parse(data)?;
        Ok((
            data,
            CP21WorldEvent {
                event,
                location,
                data: d,
                disable_relative_volume,
            },
        ))
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
crate::protocol::packets::packet!(
    CP50SetEntityVelocity,
    0x50,
    crate::protocol::ClientState::Play,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CP50SetEntityVelocity> {
        let (data, entity_id) = VarInt::parse(data)?;
        let (data, entity_velocity) = EntityVelocity::parse(data)?;
        Ok((
            data,
            CP50SetEntityVelocity {
                entity_id,
                entity_velocity,
            },
        ))
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
crate::protocol::packets::packet!(
    CP52SetExperience,
    0x52,
    crate::protocol::ClientState::Play,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CP52SetExperience> {
        let (data, experience_bar) = f32::parse(data)?;
        let (data, total_experience) = VarInt::parse(data)?;
        let (data, level) = VarInt::parse(data)?;
        Ok((
            data,
            CP52SetExperience {
                experience_bar,
                total_experience,
                level,
            },
        ))
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
crate::protocol::packets::packet!(
    CP68EntityEffect,
    0x68,
    crate::protocol::ClientState::Play,
    false,
    |data: &'data [u8]| -> crate::protocol::parsing::IResult<&'data [u8], CP68EntityEffect> {
        let (data, entity_id) = VarInt::parse(data)?;
        let (data, effect_id) = VarInt::parse(data)?;
        let (data, amplifier) = i8::parse(data)?;
        let (data, duration) = VarInt::parse(data)?;
        let (data, flags) = u8::parse(data)?;
        let is_ambient = flags & 0x01 > 0;
        let show_particles = flags & 0x02 > 0;
        let show_icon = flags & 0x04 > 0;
        let (data, has_factor_data) = bool::parse(data)?;
        // TODO: factor_codec

        Ok((
            data,
            CP68EntityEffect {
                entity_id,
                effect_id,
                amplifier,
                duration,
                is_ambient,
                show_particles,
                show_icon,
                has_factor_data,
            },
        ))
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
