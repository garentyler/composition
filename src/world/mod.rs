/// Worlds are divided into chunks.
pub mod chunks;
/// When managing a `World` encounters errors.
pub mod error;
/// Default implementations of `World`, such as `Superflat`.
pub mod generators;
/// Useful re-exports.
pub mod prelude {
    pub use super::{chunks::Chunk, World};
}

pub use crate::protocol::{blocks, entities};
pub use error::{Error, Result};

use crate::world::chunks::{Chunk, ChunkPosition};
use blocks::{Block, BlockPosition};
use entities::{Entity, EntityId, EntityPosition};
use std::path::Path;

/// A `World` abstracts away world generation, updating blocks, and saving.
#[async_trait::async_trait]
pub trait World {
    /// Get the world's name.
    fn name() -> String;
    /// Create a new world from a seed.
    fn new(seed: u128) -> Self;
    /// Load an existing world from a directory.
    async fn load<P: AsRef<Path> + Send>(world_dir: P) -> Result<Self>
    where
        Self: Sized;
    /// Save the world to a directory.
    async fn save<P: AsRef<Path> + Send>(&self, world_dir: P) -> Result<()>;

    /// Check whether a chunk is loaded or not.
    fn is_chunk_loaded(&self, chunk_pos: ChunkPosition) -> bool;
    /// Load a chunk if it's unloaded, does nothing if the chunk is already loaded.
    async fn load_chunk(&self, chunk_pos: ChunkPosition) -> Result<()>;
    /// Unload a chunk if it's loaded, does nothing if the chunk is already unloaded.
    async fn unload_chunk(&self, chunk_pos: ChunkPosition) -> Result<()>;
    /// Gets a copy of the chunk at the given `ChunkPosition`.
    async fn get_chunk(&self, chunk_pos: ChunkPosition) -> Result<Chunk>;
    /// Sets the chunk at the given `ChunkPosition`.
    async fn set_chunk(&self, chunk_pos: ChunkPosition, chunk: Chunk) -> Result<()>;

    /// Get the block at the given `BlockPosition`.
    ///
    /// Async because the containing chunk might need to be loaded.
    async fn get_block(&self, block_pos: BlockPosition) -> Result<Block>;
    /// Set the block at the given `BlockPosition`.
    ///
    /// Async because the containing chunk might need to be loaded.
    async fn set_block(&self, block_pos: BlockPosition, block: Block) -> Result<()>;

    /// Spawn an entity at the given `EntityPosition`.
    ///
    /// Async because the containing chunk might need to be loaded.
    async fn spawn_entity(&self, entity_pos: EntityPosition, entity: Entity) -> Result<EntityId>;
    /// Get a reference to the entity with the given `EntityId`.
    /// Returns Err if no entity could be found with that id.
    fn get_entity(&self, entity_id: EntityId) -> Result<&Entity>;
    /// Get a mutable reference to the entity with the given `EntityId`.
    /// Returns Err if no entity could be found with that id.
    fn get_entity_mut(&self, entity_id: EntityId) -> Result<&mut Entity>;
    /// Remove the entity with the given `EntityId`.
    ///
    /// Async because the containing chunk might need to be loaded.
    ///
    /// This should not kill the entity, it should simply remove it from processing.
    async fn remove_entity(&self, entity_id: EntityId) -> Result<()>;
}
