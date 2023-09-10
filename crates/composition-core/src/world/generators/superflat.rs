use composition_protocol::blocks::Block;

use crate::world::{
    chunks::{Chunk, ChunkPosition},
    generators::Generator,
};

/// An implementation of Minecraft's superflat world type.
pub struct Superflat {
    blocks: Vec<Block>,
}
impl Superflat {
    pub fn new(blocks: &[Block]) -> Superflat {
        Superflat {
            blocks: blocks.to_vec(),
        }
    }
}
impl Default for Superflat {
    fn default() -> Self {
        Superflat::new(&[Block::Bedrock, Block::Dirt, Block::Dirt, Block::Grass])
    }
}
impl Generator for Superflat {
    fn name(&self) -> &'static str {
        "superflat"
    }
    fn generate_chunk(&self, _: u128, chunk_position: ChunkPosition) -> Chunk {
        let mut chunk = Chunk::default();

        for (offset, block) in chunk.blocks_mut() {
            let block_position = offset.to_global_position(chunk_position);

            if block_position.y >= 0 {
                if let Some(spec) = self.blocks.get(block_position.y as usize) {
                    *block = *spec;
                }
            }
        }

        chunk
    }
}
