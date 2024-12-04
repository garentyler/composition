use super::EntityId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Frog {
    pub variant: FrogVariant,
    pub target: EntityId,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum FrogVariant {
    #[default]
    Temperate,
    // TODO: Add more frog variants
}
