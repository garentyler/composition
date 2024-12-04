use crate::protocol::mctypes::VarInt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Cat {
    pub variant: CatVariant,
    pub is_lying: bool,
    pub is_relaxed: bool,
    pub collar_color: VarInt,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum CatVariant {
    #[default]
    Black,
    // TODO: Add more cat variants
}
