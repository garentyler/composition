/// Worlds are divided into chunks.
pub mod chunks;
/// When managing a `World` encounters errors.
pub mod error;
/// Default implementations of `World`, such as `Superflat`.
pub mod generators;
/// Useful re-exports.
pub mod prelude {
    pub use crate::{chunks::Chunk, World};
}

pub use composition_protocol::{blocks, entities};
pub use error::{Error, Result};

use crate::{
    chunks::{Chunk, ChunkPosition},
    generators::Generator,
};
use composition_protocol::blocks::{Block, BlockPosition};
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::sync::RwLock;

pub struct World {
    pub name: String,
    pub seed: u128,
    chunks: Arc<RwLock<HashMap<ChunkPosition, Arc<RwLock<Chunk>>>>>,
    pub generator: Arc<RwLock<Arc<dyn Generator>>>,
}
impl World {
    /// Load an existing world from a directory.
    pub async fn load<P: AsRef<Path> + Send>(_world_directory: P) -> Result<World> {
        todo!();
    }
    /// Save the world to a directory.
    pub async fn save<P: AsRef<Path> + Send>(&self, _world_directory: P) -> Result<World> {
        todo!();
    }

    pub async fn set_generator(&self, generator: Arc<dyn Generator>) {
        *self.generator.write().await = generator;
    }

    /// Check wheter a chunk is loaded or not.
    pub async fn has_chunk_loaded(&self, chunk_position: ChunkPosition) -> bool {
        self.chunks.read().await.contains_key(&chunk_position)
    }
    /// Load a chunk if it's unloaded, doing nothing if the chunk is already loaded.
    pub async fn load_chunk(&self, chunk_position: ChunkPosition) {
        if self.has_chunk_loaded(chunk_position).await {
            return;
        }

        // Try to load the chunk from disk first.
        // TODO: Chunk loading from disk.
        // If the chunk wasn't found, generate it.
        let generator = self.generator.read().await;
        let chunk = generator.generate_chunk(self.seed, chunk_position);
        self.chunks
            .write()
            .await
            .insert(chunk_position, Arc::new(RwLock::new(chunk)));
    }
    /// Unload a chunk if it's loaded, doing nothing if the chunk is already unloaded.
    pub async fn unload_chunk(&self, chunk_position: ChunkPosition) {
        if !self.has_chunk_loaded(chunk_position).await {
            return;
        }

        // TODO: Save the chunk to disk.
        let _ = self.chunks.write().await.remove(&chunk_position);
    }
    /// Gets a copy of the chunk at the given `ChunkPosition`.
    pub async fn get_chunk(&self, chunk_position: ChunkPosition) -> Chunk {
        if !self.has_chunk_loaded(chunk_position).await {
            self.load_chunk(chunk_position).await;
        }

        // TODO: Fix potential TOCTOU race condition.

        let chunks = self.chunks.read().await;
        let chunk = chunks
            .get(&chunk_position)
            .expect("the chunk should be loaded")
            .read()
            .await;

        // TODO: Find a way to avoid cloning the chunk data, maybe pass a reference.
        chunk.clone()
    }
    /// Sets the chunk at the given `ChunkPosition`.
    pub async fn set_chunk(&self, chunk_position: ChunkPosition, chunk: Chunk) {
        let chunks = self.chunks.read().await;
        if let Some(c) = chunks.get(&chunk_position) {
            *c.write().await = chunk;
        } else {
            // Manually drop the read guard so we can gain write access.
            std::mem::drop(chunks);
            self.chunks
                .write()
                .await
                .insert(chunk_position, Arc::new(RwLock::new(chunk)));
        }
    }

    /// Gets a copy of the block at the given `BlockPosition`.
    pub async fn get_block(&self, block_position: BlockPosition) -> Block {
        let chunk_position = ChunkPosition::from(block_position);
        let chunk = self.get_chunk(chunk_position).await;
        // TODO: Find a way to avoid cloning the block.
        chunk.get_block(block_position.into()).unwrap().clone()
    }
    /// Sets the block at the given `BlockPosition`.
    pub async fn set_block(&self, block_position: BlockPosition, block: Block) {
        // If `self.get_chunk()` could return a reference,
        // we would avoid copying the entire chunk to edit one block.
        let chunk_position = ChunkPosition::from(block_position);
        let mut chunk = self.get_chunk(chunk_position).await;
        *chunk.get_block_mut(block_position.into()).unwrap() = block;
        self.set_chunk(chunk_position, chunk).await;
    }

    // TODO: Entity methods and processing.
}
