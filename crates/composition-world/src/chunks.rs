use crate::{
    blocks::{Block, BlockPosition},
    entities::{Entity, EntityId, EntityPosition},
};
use std::collections::HashMap;

/// `Chunk`s divide the world into smaller parts
/// and manage the blocks and entities within.
#[derive(Debug, Clone)]
pub struct Chunk {
    // blocks[x][y][z]
    pub blocks: [[[Block; 16]; 320]; 16],
    pub entities: HashMap<EntityId, (EntityPosition, Entity)>,
}
impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            blocks: [[[Block::default(); 16]; 320]; 16],
            entities: HashMap::new(),
        }
    }
}

/// Position for a `Chunk`.
///
/// To convert to block positions, multiply by a factor of 16.
#[derive(Debug, Copy, Clone, PartialEq, Default, Eq, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl From<BlockPosition> for ChunkPosition {
    fn from(value: BlockPosition) -> Self {
        // Divide by 16 to get the chunk.
        ChunkPosition {
            x: value.x >> 4,
            y: value.y >> 4,
            z: value.z >> 4,
        }
    }
}
impl From<EntityPosition> for ChunkPosition {
    fn from(value: EntityPosition) -> Self {
        // Divide by 16 and convert to i32.
        ChunkPosition {
            x: (value.x / 16.0) as i32,
            y: (value.y / 16.0) as i32,
            z: (value.z / 16.0) as i32,
        }
    }
}
