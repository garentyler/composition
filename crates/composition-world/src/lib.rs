pub mod chunks;
pub mod generators;

pub use composition_protocol::{blocks, entities};

use crate::chunks::ChunkPosition;
use blocks::BlockPosition;
use std::path::Path;
use thiserror::Error;

#[async_trait::async_trait]
pub trait World {
    /// Get the world's name.
    fn name() -> String;
    /// Create a new world.
    fn new(seed: u128) -> Self;
    /// Load an existing world.
    async fn load_from_dir<P: AsRef<Path> + Send>(world_dir: P) -> Result<Self>
    where
        Self: Sized;
    /// Save the world to a directory.
    async fn save_to_dir<P: AsRef<Path> + Send>(&self, world_dir: P) -> Result<()>;

    async fn is_chunk_loaded(&self, chunk_pos: ChunkPosition) -> bool;
    async fn load_chunk(&self, chunk_pos: ChunkPosition) -> Result<()>;
    async fn unload_chunk(&self, chunk_pos: ChunkPosition) -> Result<()>;
    async fn get_chunk(&self, chunk_pos: ChunkPosition) -> Result<chunks::Chunk>;
    async fn set_chunk(&self, chunk_pos: ChunkPosition, chunk: chunks::Chunk) -> Result<()>;

    // Getting/setting blocks requires async because the chunk might not be loaded.
    async fn get_block(&self, block_pos: BlockPosition) -> Result<blocks::Block>;
    async fn set_block(&self, block_pos: BlockPosition, block: blocks::Block) -> Result<()>;

    // Spawning/removing entities requires async because the chunk might not be loaded.
    async fn spawn_entity(
        &self,
        entity_pos: entities::EntityPosition,
        entity: entities::Entity,
    ) -> Result<entities::EntityId>;
    fn get_entity(&self, entity_id: entities::EntityId) -> Result<&entities::Entity>;
    fn get_entity_mut(&self, entity_id: entities::EntityId) -> Result<&mut entities::Entity>;
    async fn remove_entity(&self, entity_id: entities::EntityId) -> Result<()>;
}

#[derive(Error, Debug)]
pub enum WorldError {
    #[error("the given position was out of bounds")]
    OutOfBounds,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
pub type Result<T> = std::result::Result<T, WorldError>;
