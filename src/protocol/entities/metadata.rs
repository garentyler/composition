use crate::protocol::{
    blocks::BlockFace,
    mctypes::{Chat, Position, Uuid, VarInt},
};

pub type EntityMetadata = Vec<EntityMetadataEntry>;

#[derive(Debug, Clone, PartialEq)]
pub struct EntityMetadataEntry {
    pub index: u8,
    pub kind: EntityMetadataEntryKind,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum EntityMetadataEntryKind {
    Byte(u8) = 0,
    VarInt(VarInt) = 1,
    // TODO: Add VarLong type
    VarLong(VarInt) = 2,
    Float(f32) = 3,
    String(String) = 4,
    Chat(Chat) = 5,
    OptionalChat(Option<Chat>) = 6,
    // TODO: Add Slot type
    Slot(()) = 7,
    Boolean(bool) = 8,
    Rotation {
        x: f32,
        y: f32,
        z: f32,
    } = 9,
    Position(Position) = 10,
    OptionalPosition(Option<Position>) = 11,
    Direction(BlockFace) = 12,
    OptionalUuid(Uuid) = 13,
    BlockId(VarInt) = 14,
    // 0 or None means air
    OptionalBlockId(Option<VarInt>) = 15,
    // TODO: Add NBT type
    Nbt(()) = 16,
    // TODO: Add Particle type
    Particle(()) = 17,
    VillagerData {
        biome: super::villager::VillagerBiome,
        profession: super::villager::VillagerProfession,
        level: VarInt,
    } = 18,
    // Used for entity ids
    OptionalVarInt(VarInt) = 19,
    Pose(EntityPose) = 20,
    CatVariant(super::cat::CatVariant) = 21,
    FrogVariant(super::frog::FrogVariant) = 22,
    // TODO: Add dimension id
    OptionalGlobalPosition((), Position) = 23,
    // TODO: Add painting variant
    PaintingVariant(()) = 24,
    #[cfg(feature = "update_1_20")]
    SnifferState(super::sniffer::SnifferState) = 25,
    Vector3 {
        x: f32,
        y: f32,
        z: f32,
    } = 26,
    Quaternion {
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    } = 27,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum EntityPose {
    #[default]
    Standing = 0,
    FallFlying = 1,
    Sleeping = 2,
    Swimming = 3,
    SpinAttack = 4,
    Sneaking = 5,
    LongJumping = 6,
    Dying = 7,
    Croaking = 8,
    UsingTongue = 9,
    Sitting = 10,
    Roaring = 11,
    Sniffing = 12,
    Emerging = 13,
    Digging = 14,
}
