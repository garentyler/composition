use crate::chunks::{Chunk, ChunkPosition};

/// An implementation of Minecraft's superflat world type.
pub mod superflat;

pub trait Generator {
    fn name(&self) -> &'static str;
    fn generate_chunk(&self, world_seed: u128, chunk_position: ChunkPosition) -> Chunk;
}
