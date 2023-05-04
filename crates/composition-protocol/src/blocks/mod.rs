pub type BlockId = &'static str;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl BlockPosition {
    pub fn as_chunk_offset(&self) -> (usize, usize, usize) {
        (
            (self.x % 16) as usize,
            (self.y % 16) as usize,
            (self.z % 16) as usize,
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum BlockFace {
    Bottom = 0,
    Top = 1,
    #[default]
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Block {
    #[default]
    Air,
    // TODO
}
