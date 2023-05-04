pub mod cat;
pub mod frog;
pub mod metadata;
pub mod particle;
pub mod player;
#[cfg(feature = "update_1_20")]
pub mod sniffer;
pub mod villager;

use crate::{
    blocks::BlockPosition,
    mctypes::{Chat, Uuid, VarInt},
};
use composition_parsing::{parsable::Parsable, ParseResult};

pub type EntityId = VarInt;
pub type EntityUuid = Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct EntityPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Parsable for EntityPosition {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, x) = f64::parse(data)?;
        let (data, y) = f64::parse(data)?;
        let (data, z) = f64::parse(data)?;
        Ok((data, EntityPosition { x, y, z }))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend(self.x.serialize());
        output.extend(self.y.serialize());
        output.extend(self.z.serialize());
        output
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct EntityRotation {
    pub pitch: u8,
    pub yaw: u8,
}
impl Parsable for EntityRotation {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, pitch) = u8::parse(data)?;
        let (data, yaw) = u8::parse(data)?;
        Ok((data, EntityRotation { pitch, yaw }))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend(self.pitch.serialize());
        output.extend(self.yaw.serialize());
        output
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct EntityVelocity {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
impl Parsable for EntityVelocity {
    #[tracing::instrument]
    fn parse(data: &[u8]) -> ParseResult<'_, Self> {
        let (data, x) = i16::parse(data)?;
        let (data, y) = i16::parse(data)?;
        let (data, z) = i16::parse(data)?;
        Ok((data, EntityVelocity { x, y, z }))
    }
    #[tracing::instrument]
    fn serialize(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend(self.x.serialize());
        output.extend(self.y.serialize());
        output.extend(self.z.serialize());
        output
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Entity {
    pub position: EntityPosition,
    pub velocity: EntityVelocity,
    pub is_on_fire: bool,
    pub is_crouching: bool,
    pub is_sprinting: bool,
    pub is_swimming: bool,
    pub is_invisible: bool,
    pub is_glowing: bool,
    pub is_elytra_flying: bool,
    pub custom_name: Option<Chat>,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LivingEntity {
    pub is_hand_active: bool,
    pub main_hand: bool,
    pub in_riptide_spin_attack: bool,
    pub health: f32,
    pub potion_effect_color: Option<VarInt>,
    pub is_potion_effect_ambient: bool,
    pub arrow_count: VarInt,
    pub bee_stingers: VarInt,
    pub currently_sleeping_bed_position: Option<BlockPosition>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Mob {
    pub has_ai: bool,
    pub is_left_handed: bool,
    pub is_aggressive: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct PathfinderMob;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AgeableMob {
    pub is_baby: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Animal;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct TameableAnimal {
    pub is_sitting: bool,
    pub is_tamed: bool,
    pub owner: Option<Uuid>,
}
