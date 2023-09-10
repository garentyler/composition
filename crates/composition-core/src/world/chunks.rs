use composition_protocol::{
    blocks::{Block, BlockPosition},
    entities::{Entity, EntityId, EntityPosition},
};
use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

/// `Chunk`s divide the world into smaller parts
/// and manage the blocks and entities within.
#[derive(Debug, Clone)]
pub struct Chunk {
    // blocks[x][y][z]
    blocks: [[[Block; Chunk::SIZE]; Chunk::SIZE]; Chunk::SIZE],
    #[allow(dead_code)]
    entities: HashMap<EntityId, (EntityPosition, Entity)>,
}
impl Chunk {
    pub const SIZE: usize = 16;

    pub fn get_block(&self, block_offset: BlockOffset) -> Option<&Block> {
        if block_offset.is_valid() {
            Some(&self[block_offset])
        } else {
            None
        }
    }
    pub fn get_block_mut(&mut self, block_offset: BlockOffset) -> Option<&mut Block> {
        if block_offset.is_valid() {
            Some(&mut self[block_offset])
        } else {
            None
        }
    }
    pub fn blocks(&self) -> impl Iterator<Item = (BlockOffset, &'_ Block)> {
        self.blocks.iter().enumerate().flat_map(|(x, blocks)| {
            blocks.iter().enumerate().flat_map(move |(y, blocks)| {
                blocks
                    .iter()
                    .enumerate()
                    .map(move |(z, block)| (BlockOffset { x, y, z }, block))
            })
        })
    }
    pub fn blocks_mut(&mut self) -> impl Iterator<Item = (BlockOffset, &'_ mut Block)> {
        self.blocks.iter_mut().enumerate().flat_map(|(x, blocks)| {
            blocks.iter_mut().enumerate().flat_map(move |(y, blocks)| {
                blocks
                    .iter_mut()
                    .enumerate()
                    .map(move |(z, block)| (BlockOffset { x, y, z }, block))
            })
        })
    }
}
impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            blocks: [[[Block::default(); Chunk::SIZE]; Chunk::SIZE]; Chunk::SIZE],
            entities: HashMap::new(),
        }
    }
}
impl Index<BlockOffset> for Chunk {
    type Output = Block;

    fn index(&self, index: BlockOffset) -> &Self::Output {
        &self.blocks[index.x][index.z][index.y]
    }
}
impl IndexMut<BlockOffset> for Chunk {
    fn index_mut(&mut self, index: BlockOffset) -> &mut Self::Output {
        &mut self.blocks[index.x][index.z][index.y]
    }
}

/// The offset of a block within a chunk.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct BlockOffset {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
impl BlockOffset {
    pub fn is_valid(&self) -> bool {
        self.x < Chunk::SIZE && self.y < Chunk::SIZE && self.z < Chunk::SIZE
    }
    pub fn to_global_position(self, chunk_position: ChunkPosition) -> BlockPosition {
        BlockPosition {
            x: (chunk_position.x * Chunk::SIZE as i32) + (self.x as i32),
            y: (chunk_position.y * Chunk::SIZE as i32) + (self.y as i32),
            z: (chunk_position.z * Chunk::SIZE as i32) + (self.z as i32),
        }
    }
}
impl From<BlockPosition> for BlockOffset {
    fn from(value: BlockPosition) -> Self {
        // Modulo by the chunk size.
        BlockOffset {
            x: (value.x % (Chunk::SIZE as i32)) as usize,
            y: (value.y % (Chunk::SIZE as i32)) as usize,
            z: (value.z % (Chunk::SIZE as i32)) as usize,
        }
    }
}

/// Position for a `Chunk`.
///
/// To convert to block positions, multiply by `Chunk::SIZE`.
#[derive(Debug, Copy, Clone, PartialEq, Default, Eq, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl From<BlockPosition> for ChunkPosition {
    fn from(value: BlockPosition) -> Self {
        // Divide by the chunk size.
        ChunkPosition {
            x: value.x / (Chunk::SIZE as i32),
            y: value.y / (Chunk::SIZE as i32),
            z: value.z / (Chunk::SIZE as i32),
        }
    }
}
impl From<EntityPosition> for ChunkPosition {
    fn from(value: EntityPosition) -> Self {
        // Divide by the chunk size and convert to i32.
        ChunkPosition {
            x: (value.x / (Chunk::SIZE as f64)) as i32,
            y: (value.y / (Chunk::SIZE as f64)) as i32,
            z: (value.z / (Chunk::SIZE as f64)) as i32,
        }
    }
}
