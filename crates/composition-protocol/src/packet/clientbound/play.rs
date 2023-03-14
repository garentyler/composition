use crate::{
    packet::{GenericPacket, Packet, PacketId},
    util::{
        parse_json, parse_uuid, parse_varint, serialize_json, serialize_uuid, serialize_varint,
        Position,
    },
    Chat, Difficulty, Uuid,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CP00SpawnEntity {
    entity_id: i32,
    entity_uuid: Uuid,
    kind: i32,
    x: f64,
    y: f64,
    z: f64,
    pitch: u8,
    yaw: u8,
    head_yaw: u8,
    data: i32,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}
impl Packet for CP00SpawnEntity {
    fn id() -> PacketId {
        0x00
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, entity_id) = parse_varint(data)?;
        let (data, entity_uuid) = parse_uuid(data)?;
        let (data, kind) = parse_varint(data)?;
        let (data, x) = nom::number::streaming::be_f64(data)?;
        let (data, y) = nom::number::streaming::be_f64(data)?;
        let (data, z) = nom::number::streaming::be_f64(data)?;
        let (data, t) = nom::bytes::streaming::take(3usize)(data)?;
        let (data, d) = parse_varint(data)?;
        let (data, velocity_x) = nom::number::streaming::be_i16(data)?;
        let (data, velocity_y) = nom::number::streaming::be_i16(data)?;
        let (data, velocity_z) = nom::number::streaming::be_i16(data)?;
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
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(self.entity_id));
        output.extend_from_slice(&serialize_uuid(&self.entity_uuid));
        output.extend_from_slice(&serialize_varint(self.kind));
        output.extend_from_slice(&self.x.to_be_bytes());
        output.extend_from_slice(&self.y.to_be_bytes());
        output.extend_from_slice(&self.z.to_be_bytes());
        output.push(self.pitch);
        output.push(self.yaw);
        output.push(self.head_yaw);
        output.extend_from_slice(&serialize_varint(self.data));
        output.extend_from_slice(&self.velocity_x.to_be_bytes());
        output.extend_from_slice(&self.velocity_y.to_be_bytes());
        output.extend_from_slice(&self.velocity_z.to_be_bytes());
        output
    }
}
impl From<CP00SpawnEntity> for GenericPacket {
    fn from(value: CP00SpawnEntity) -> Self {
        GenericPacket::CP00SpawnEntity(value)
    }
}
impl TryFrom<GenericPacket> for CP00SpawnEntity {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CP00SpawnEntity(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CP0BChangeDifficulty {
    difficulty: Difficulty,
    is_locked: bool,
}
impl Packet for CP0BChangeDifficulty {
    fn id() -> PacketId {
        0x0b
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, difficulty) = nom::number::streaming::be_u8(data)?;
        let difficulty: Difficulty = difficulty
            .try_into()
            .expect("TODO: handle incorrect difficulty");
        let (data, is_locked) = nom::number::streaming::be_u8(data)?;
        let is_locked = is_locked > 0;
        Ok((
            data,
            CP0BChangeDifficulty {
                difficulty,
                is_locked,
            },
        ))
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.push(self.difficulty as u8);
        output.push(if self.is_locked { 0x01 } else { 0x00 });
        output
    }
}
impl From<CP0BChangeDifficulty> for GenericPacket {
    fn from(value: CP0BChangeDifficulty) -> Self {
        GenericPacket::CP0BChangeDifficulty(value)
    }
}
impl TryFrom<GenericPacket> for CP0BChangeDifficulty {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CP0BChangeDifficulty(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CP17Disconnect {
    reason: Chat,
}
impl Packet for CP17Disconnect {
    fn id() -> PacketId {
        0x17
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, reason) = parse_json(data)?;
        Ok((data, CP17Disconnect { reason }))
    }
    fn serialize_body(&self) -> Vec<u8> {
        serialize_json(&self.reason)
    }
}
impl From<CP17Disconnect> for GenericPacket {
    fn from(value: CP17Disconnect) -> Self {
        GenericPacket::CP17Disconnect(value)
    }
}
impl TryFrom<GenericPacket> for CP17Disconnect {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CP17Disconnect(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP1FKeepAlive {
    payload: i64,
}
impl Packet for CP1FKeepAlive {
    fn id() -> PacketId {
        0x1f
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, payload) = nom::number::streaming::be_i64(data)?;
        Ok((data, CP1FKeepAlive { payload }))
    }
    fn serialize_body(&self) -> Vec<u8> {
        self.payload.to_be_bytes().to_vec()
    }
}
impl From<CP1FKeepAlive> for GenericPacket {
    fn from(value: CP1FKeepAlive) -> Self {
        GenericPacket::CP1FKeepAlive(value)
    }
}
impl TryFrom<GenericPacket> for CP1FKeepAlive {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CP1FKeepAlive(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CP21WorldEvent {
    event: i32,
    location: Position,
    data: i32,
    disable_relative_volume: bool,
}
impl Packet for CP21WorldEvent {
    fn id() -> PacketId {
        0x21
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, event) = nom::number::streaming::be_i32(data)?;
        let (data, location) = Position::parse(data)?;
        let (data, d) = nom::number::streaming::be_i32(data)?;
        let (data, disable_relative_volume) = nom::bytes::streaming::take(1usize)(data)?;
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
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&self.event.to_be_bytes());
        output.extend_from_slice(&self.location.serialize());
        output.extend_from_slice(&self.data.to_be_bytes());
        output.push(if self.disable_relative_volume {
            0x01
        } else {
            0x00
        });
        output
    }
}
impl From<CP21WorldEvent> for GenericPacket {
    fn from(value: CP21WorldEvent) -> Self {
        GenericPacket::CP21WorldEvent(value)
    }
}
impl TryFrom<GenericPacket> for CP21WorldEvent {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CP21WorldEvent(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CP50SetEntityVelocity {
    entity_id: i32,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}
impl Packet for CP50SetEntityVelocity {
    fn id() -> PacketId {
        0x50
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, entity_id) = parse_varint(data)?;
        let (data, velocity_x) = nom::number::streaming::be_i16(data)?;
        let (data, velocity_y) = nom::number::streaming::be_i16(data)?;
        let (data, velocity_z) = nom::number::streaming::be_i16(data)?;
        Ok((
            data,
            CP50SetEntityVelocity {
                entity_id,
                velocity_x,
                velocity_y,
                velocity_z,
            },
        ))
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(self.entity_id));
        output.extend_from_slice(&self.velocity_x.to_be_bytes());
        output.extend_from_slice(&self.velocity_y.to_be_bytes());
        output.extend_from_slice(&self.velocity_z.to_be_bytes());
        output
    }
}
impl From<CP50SetEntityVelocity> for GenericPacket {
    fn from(value: CP50SetEntityVelocity) -> Self {
        GenericPacket::CP50SetEntityVelocity(value)
    }
}
impl TryFrom<GenericPacket> for CP50SetEntityVelocity {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CP50SetEntityVelocity(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CP52SetExperience {
    experience_bar: f32,
    total_experience: i32,
    level: i32,
}
impl Packet for CP52SetExperience {
    fn id() -> PacketId {
        0x52
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, experience_bar) = nom::number::streaming::be_f32(data)?;
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
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&self.experience_bar.to_be_bytes());
        output.extend_from_slice(&serialize_varint(self.total_experience));
        output.extend_from_slice(&serialize_varint(self.level));
        output
    }
}
impl From<CP52SetExperience> for GenericPacket {
    fn from(value: CP52SetExperience) -> Self {
        GenericPacket::CP52SetExperience(value)
    }
}
impl TryFrom<GenericPacket> for CP52SetExperience {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CP52SetExperience(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CP68EntityEffect {
    entity_id: i32,
    effect_id: i32,
    amplifier: i8,
    duration: i32,
    is_ambient: bool,
    show_particles: bool,
    show_icon: bool,
    has_factor_data: bool,
    // TODO: factor_codec: NBT
}
impl Packet for CP68EntityEffect {
    fn id() -> PacketId {
        0x68
    }
    fn client_state() -> crate::ClientState {
        crate::ClientState::Play
    }
    fn serverbound() -> bool {
        false
    }

    fn parse_body(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (data, entity_id) = parse_varint(data)?;
        let (data, effect_id) = parse_varint(data)?;
        let (data, amplifier) = nom::number::streaming::be_i8(data)?;
        let (data, duration) = parse_varint(data)?;
        let (data, flags) = nom::number::streaming::be_i8(data)?;
        let is_ambient = flags & 0x01 > 0;
        let show_particles = flags & 0x02 > 0;
        let show_icon = flags & 0x04 > 0;
        let (data, has_factor_data) = nom::number::streaming::be_u8(data)?;
        let has_factor_data = has_factor_data > 0;
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
    }
    fn serialize_body(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(&serialize_varint(self.entity_id));
        output.extend_from_slice(&serialize_varint(self.effect_id));
        output.push(self.amplifier as u8);
        output.extend_from_slice(&serialize_varint(self.duration));
        let mut flags = 0x00i8;
        if self.is_ambient {
            flags |= 0x01;
        }
        if self.show_particles {
            flags |= 0x02;
        }
        if self.show_icon {
            flags |= 0x04;
        }
        output.push(flags as u8);
        // TODO: factor_codec
        output
    }
}
impl From<CP68EntityEffect> for GenericPacket {
    fn from(value: CP68EntityEffect) -> Self {
        GenericPacket::CP68EntityEffect(value)
    }
}
impl TryFrom<GenericPacket> for CP68EntityEffect {
    type Error = ();

    fn try_from(value: GenericPacket) -> Result<Self, Self::Error> {
        match value {
            GenericPacket::CP68EntityEffect(packet) => Ok(packet),
            _ => Err(()),
        }
    }
}
