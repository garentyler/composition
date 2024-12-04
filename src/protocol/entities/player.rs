use crate::protocol::mctypes::VarInt;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Player {
    pub additional_hearts: f32,
    pub score: VarInt,
    pub skin_parts: PlayerSkinParts,
    pub right_handed: bool,
    // TODO: NBT data
    pub left_shoulder_entity: (),
    // TODO: NBT data
    pub right_shoulder_entity: (),
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct PlayerSkinParts {
    pub cape_enabled: bool,
    pub jacket_enabled: bool,
    pub left_sleeve_enabled: bool,
    pub right_sleeve_enabled: bool,
    pub left_pant_leg_enabled: bool,
    pub right_pant_leg_enabled: bool,
    pub hat_enabled: bool,
}
