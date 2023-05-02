use crate::{util::*, Chat, Difficulty, ProtocolError, Uuid};
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP00SpawnEntity {
    pub entity_id: i32,
    pub entity_uuid: Uuid,
    pub kind: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: u8,
    pub yaw: u8,
    pub head_yaw: u8,
    pub data: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}
crate::packet::packet!(
    CP00SpawnEntity,
    0x00,
    crate::ClientState::Play,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CP00SpawnEntity> {
        let (data, entity_id) = parse_varint(data)?;
        let (data, entity_uuid) = parse_uuid(data)?;
        let (data, kind) = parse_varint(data)?;
        let (data, mut bytes) = take_bytes(8)(data)?;
        let x = bytes
            .read_f64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(8)(data)?;
        let y = bytes
            .read_f64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(8)(data)?;
        let z = bytes
            .read_f64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, t) = take_bytes(3usize)(data)?;
        let (data, d) = parse_varint(data)?;
        let (data, mut bytes) = take_bytes(2)(data)?;
        let velocity_x = bytes
            .read_i16::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(2)(data)?;
        let velocity_y = bytes
            .read_i16::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(2)(data)?;
        let velocity_z = bytes
            .read_i16::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;

        Ok((
            data,
            CP00SpawnEntity {
                entity_id,
                entity_uuid,
                kind,
                x,
                y,
                z,
                pitch: t[0],
                yaw: t[1],
                head_yaw: t[2],
                data: d,
                velocity_x,
                velocity_y,
                velocity_z,
            },
        ))
    },
    |packet: &CP00SpawnEntity| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(packet.entity_id));
        output.extend_from_slice(&serialize_uuid(&packet.entity_uuid));
        output.extend_from_slice(&serialize_varint(packet.kind));
        output.extend_from_slice(&packet.x.to_be_bytes());
        output.extend_from_slice(&packet.y.to_be_bytes());
        output.extend_from_slice(&packet.z.to_be_bytes());
        output.push(packet.pitch);
        output.push(packet.yaw);
        output.push(packet.head_yaw);
        output.extend_from_slice(&serialize_varint(packet.data));
        output.extend_from_slice(&packet.velocity_x.to_be_bytes());
        output.extend_from_slice(&packet.velocity_y.to_be_bytes());
        output.extend_from_slice(&packet.velocity_z.to_be_bytes());
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP0BChangeDifficulty {
    pub difficulty: Difficulty,
    pub is_locked: bool,
}
crate::packet::packet!(
    CP0BChangeDifficulty,
    0x0b,
    crate::ClientState::Play,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CP0BChangeDifficulty> {
        let (data, difficulty) = take_bytes(1)(data)?;
        let difficulty: Difficulty = difficulty[0]
            .try_into()
            .expect("TODO: handle incorrect difficulty");
        let (data, is_locked) = take_bytes(1)(data)?;
        let is_locked = is_locked[0] > 0;
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
        output.push(packet.difficulty as u8);
        output.push(if packet.is_locked { 0x01 } else { 0x00 });
        output
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct CP17Disconnect {
    pub reason: Chat,
}
crate::packet::packet!(
    CP17Disconnect,
    0x17,
    crate::ClientState::Play,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CP17Disconnect> {
        let (data, reason) = parse_json(data)?;
        Ok((data, CP17Disconnect { reason }))
    },
    |packet: &CP17Disconnect| -> Vec<u8> { serialize_json(&packet.reason) }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP1FKeepAlive {
    pub payload: i64,
}
crate::packet::packet!(
    CP1FKeepAlive,
    0x1f,
    crate::ClientState::Play,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CP1FKeepAlive> {
        let (data, mut bytes) = take_bytes(8)(data)?;
        let payload = bytes
            .read_i64::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        Ok((data, CP1FKeepAlive { payload }))
    },
    |packet: &CP1FKeepAlive| -> Vec<u8> { packet.payload.to_be_bytes().to_vec() }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP21WorldEvent {
    pub event: i32,
    pub location: Position,
    pub data: i32,
    pub disable_relative_volume: bool,
}
crate::packet::packet!(
    CP21WorldEvent,
    0x21,
    crate::ClientState::Play,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CP21WorldEvent> {
        let (data, mut bytes) = take_bytes(4)(data)?;
        let event = bytes
            .read_i32::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, location) = Position::parse(data)?;
        let (data, mut bytes) = take_bytes(4)(data)?;
        let d = bytes
            .read_i32::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, disable_relative_volume) = take_bytes(1usize)(data)?;
        let disable_relative_volume = disable_relative_volume == [0x01];
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
        output.extend_from_slice(&packet.event.to_be_bytes());
        output.extend_from_slice(&packet.location.serialize());
        output.extend_from_slice(&packet.data.to_be_bytes());
        output.push(if packet.disable_relative_volume {
            0x01
        } else {
            0x00
        });
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP50SetEntityVelocity {
    pub entity_id: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}
crate::packet::packet!(
    CP50SetEntityVelocity,
    0x50,
    crate::ClientState::Play,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CP50SetEntityVelocity> {
        let (data, entity_id) = parse_varint(data)?;
        let (data, mut bytes) = take_bytes(2)(data)?;
        let velocity_x = bytes
            .read_i16::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(2)(data)?;
        let velocity_y = bytes
            .read_i16::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, mut bytes) = take_bytes(2)(data)?;
        let velocity_z = bytes
            .read_i16::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        Ok((
            data,
            CP50SetEntityVelocity {
                entity_id,
                velocity_x,
                velocity_y,
                velocity_z,
            },
        ))
    },
    |packet: &CP50SetEntityVelocity| -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(packet.entity_id));
        output.extend_from_slice(&packet.velocity_x.to_be_bytes());
        output.extend_from_slice(&packet.velocity_y.to_be_bytes());
        output.extend_from_slice(&packet.velocity_z.to_be_bytes());
        output
    }
);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP52SetExperience {
    pub experience_bar: f32,
    pub total_experience: i32,
    pub level: i32,
}
crate::packet::packet!(
    CP52SetExperience,
    0x52,
    crate::ClientState::Play,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CP52SetExperience> {
        let (data, mut bytes) = take_bytes(4)(data)?;
        let experience_bar = bytes
            .read_f32::<BigEndian>()
            .map_err(|_| ProtocolError::NotEnoughData)?;
        let (data, total_experience) = parse_varint(data)?;
        let (data, level) = parse_varint(data)?;
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
        output.extend_from_slice(&packet.experience_bar.to_be_bytes());
        output.extend_from_slice(&serialize_varint(packet.total_experience));
        output.extend_from_slice(&serialize_varint(packet.level));
        output
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct CP68EntityEffect {
    pub entity_id: i32,
    pub effect_id: i32,
    pub amplifier: i8,
    pub duration: i32,
    pub is_ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    pub has_factor_data: bool,
    // TODO: pub factor_codec: NBT
}
crate::packet::packet!(
    CP68EntityEffect,
    0x68,
    crate::ClientState::Play,
    false,
    |data: &'data [u8]| -> ParseResult<'data, CP68EntityEffect> {
        let (data, entity_id) = parse_varint(data)?;
        let (data, effect_id) = parse_varint(data)?;
        let (data, amplifier) = take_bytes(1)(data)?;
        let amplifier = amplifier[0] as i8;
        let (data, duration) = parse_varint(data)?;
        let (data, flags) = take_bytes(1)(data)?;
        let flags = flags[0] as i8;
        let is_ambient = flags & 0x01 > 0;
        let show_particles = flags & 0x02 > 0;
        let show_icon = flags & 0x04 > 0;
        let (data, has_factor_data) = take_bytes(1)(data)?;
        let has_factor_data = has_factor_data[0] > 0;
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
        output.extend_from_slice(&serialize_varint(packet.entity_id));
        output.extend_from_slice(&serialize_varint(packet.effect_id));
        output.push(packet.amplifier as u8);
        output.extend_from_slice(&serialize_varint(packet.duration));
        let mut flags = 0x00i8;
        if packet.is_ambient {
            flags |= 0x01;
        }
        if packet.show_particles {
            flags |= 0x02;
        }
        if packet.show_icon {
            flags |= 0x04;
        }
        output.push(flags as u8);
        // TODO: factor_codec
        output
    }
);
