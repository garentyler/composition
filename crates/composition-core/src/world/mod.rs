/// Worlds are divided into chunks.
pub mod chunks;
/// Some default world generators, such as superflat.
pub mod generators;

use crate::error::Result;
use chunks::{Chunk, ChunkPosition};
use composition_protocol::blocks::{Block, BlockPosition};
use generators::Generator;
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::sync::{mpsc, oneshot, Mutex, RwLock};
use tokio_util::sync::CancellationToken;

pub struct World {
    pub name: String,
    pub seed: u128,
    // The outer Arc<RwLock<_>> is to allow multiple chunks to be accessed at the same time, while still allowing loading and unloading when locked.
    chunks: Arc<RwLock<HashMap<ChunkPosition, Arc<RwLock<Chunk>>>>>,
    pub generator: Arc<dyn Generator>,
    command_channel: (
        mpsc::Sender<WorldCommand>,
        Mutex<mpsc::Receiver<WorldCommand>>,
    ),
    running: CancellationToken,
}
impl World {
    pub fn new(name: &str, seed: u128, generator: Option<Arc<dyn Generator>>) -> World {
        // TODO: Test different channel sizes, 256 was chosen arbitrarily
        let (command_sender, command_receiver) = mpsc::channel(256);
        let command_receiver = Mutex::new(command_receiver);

        World {
            name: name.to_string(),
            seed,
            chunks: Arc::new(RwLock::new(HashMap::new())),
            generator: generator.unwrap_or(Arc::new(generators::superflat::Superflat::default())),
            command_channel: (command_sender, command_receiver),
            running: CancellationToken::new(),
        }
    }
    /// Load an existing world from a directory.
    pub async fn load<P: AsRef<Path> + Send>(_world_directory: P) -> Result<World> {
        todo!();
    }
    /// Save the world to a directory.
    pub async fn save<P: AsRef<Path> + Send>(&self, _world_directory: P) -> Result<World> {
        todo!();
    }

    /// Get a handle to the world that can send world commands asynchronously.
    pub fn handle(&self) -> WorldHandle {
        WorldHandle {
            command_sender: self.command_channel.0.clone(),
            running: self.running.clone(),
        }
    }
    /// Process world commands until completion.
    /// Can be interrupted by cancelling `self.running`.
    pub async fn run(self) {
        let world = Arc::new(self);

        let mut tasks = Vec::new();
        loop {
            let mut command_receiver = world.command_channel.1.lock().await;

            tokio::select! {
                _ = world.running.cancelled() => break,
                command = command_receiver.recv() => {
                    let Some(command) = command else { break };
                    let world = world.clone();
                    tasks.push(tokio::spawn(async move {
                        match command {
                            WorldCommand::IsChunkLoaded { chunk_position, responder } => responder.send(world.has_chunk_loaded(chunk_position).await).unwrap(),
                            WorldCommand::LoadChunk { chunk_position } => world.load_chunk(chunk_position).await,
                            WorldCommand::UnloadChunk { chunk_position } => world.unload_chunk(chunk_position).await,
                            WorldCommand::GetChunk { chunk_position, responder } => responder.send(world.get_chunk(chunk_position).await).unwrap(),
                            WorldCommand::GetBlock { block_position, responder } => responder.send(world.get_block(block_position).await).unwrap(),
                            WorldCommand::SetBlock { block_position, block } => world.set_block(block_position, block).await,
                        }
                    }));
                }
            }

            tasks.retain(|task| !task.is_finished());
        }

        // Finish executing any server commands.
        futures::future::join_all(tasks).await;
        // This shouldn't fail - we just merged all of the other tasks that have a reference to `world`.
        let world = Arc::into_inner(world).unwrap();
        world.shutdown().await;
    }
    async fn shutdown(self) {
        // TODO: Save the world to a directory.
    }

    /// Check whether a chunk is loaded or not.
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
        self.chunks.write().await.insert(
            chunk_position,
            Arc::new(RwLock::new(
                self.generator.generate_chunk(self.seed, chunk_position),
            )),
        );
    }
    /// Unload a chunk if it's loaded, doing nothing if the chunk is already unloaded.
    pub async fn unload_chunk(&self, chunk_position: ChunkPosition) {
        if !self.has_chunk_loaded(chunk_position).await {
            return;
        }

        // TODO: Save the chunk to disk.
        let _ = self.chunks.write().await.remove(&chunk_position);
    }
    /// Gets a reference to the chunk at the given `ChunkPosition`.
    pub async fn get_chunk(&self, chunk_position: ChunkPosition) -> Arc<RwLock<Chunk>> {
        if !self.has_chunk_loaded(chunk_position).await {
            self.load_chunk(chunk_position).await;
        }

        self.chunks
            .read()
            .await
            .get(&chunk_position)
            .expect("the chunk should be loaded")
            .clone()
    }
    /// Gets a copy of the block at the given `BlockPosition`.
    pub async fn get_block(&self, block_position: BlockPosition) -> Block {
        let chunk_position = ChunkPosition::from(block_position);
        let chunk = self.get_chunk(chunk_position).await;
        let chunk = chunk.read().await;
        *chunk.get_block(block_position.into()).unwrap()
    }
    /// Sets the block at the given `BlockPosition`
    pub async fn set_block(&self, block_position: BlockPosition, block: Block) {
        let chunk_position = ChunkPosition::from(block_position);
        let chunk = self.get_chunk(chunk_position).await;
        *chunk
            .write()
            .await
            .get_block_mut(block_position.into())
            .unwrap() = block;
    }

    // TODO: Entity methods and processing.
}

#[derive(Debug)]
pub enum WorldCommand {
    IsChunkLoaded {
        chunk_position: ChunkPosition,
        responder: oneshot::Sender<bool>,
    },
    LoadChunk {
        chunk_position: ChunkPosition,
    },
    UnloadChunk {
        chunk_position: ChunkPosition,
    },
    GetChunk {
        chunk_position: ChunkPosition,
        responder: oneshot::Sender<Arc<RwLock<Chunk>>>,
    },
    GetBlock {
        block_position: BlockPosition,
        responder: oneshot::Sender<Block>,
    },
    SetBlock {
        block_position: BlockPosition,
        block: Block,
    },
}

#[derive(Clone, Debug)]
pub struct WorldHandle {
    command_sender: mpsc::Sender<WorldCommand>,
    running: CancellationToken,
}
impl WorldHandle {
    pub async fn has_chunk_loaded(&self, chunk_position: ChunkPosition) -> bool {
        let (responder, response) = oneshot::channel();
        self.command_sender
            .send(WorldCommand::IsChunkLoaded {
                chunk_position,
                responder,
            })
            .await
            .unwrap();
        response.await.unwrap()
    }
    pub async fn load_chunk(&self, chunk_position: ChunkPosition) {
        self.command_sender
            .send(WorldCommand::LoadChunk { chunk_position })
            .await
            .unwrap();
    }
    pub async fn unload_chunk(&self, chunk_position: ChunkPosition) {
        self.command_sender
            .send(WorldCommand::UnloadChunk { chunk_position })
            .await
            .unwrap();
    }
    async fn get_chunk_reference(&self, chunk_position: ChunkPosition) -> Arc<RwLock<Chunk>> {
        let (responder, response) = oneshot::channel();
        self.command_sender
            .send(WorldCommand::GetChunk {
                chunk_position,
                responder,
            })
            .await
            .unwrap();
        response.await.unwrap()
    }
    pub async fn get_chunk(&self, chunk_position: ChunkPosition) -> Chunk {
        self.get_chunk_reference(chunk_position)
            .await
            .read()
            .await
            .clone()
    }
    pub async fn set_chunk(&self, chunk_position: ChunkPosition, chunk: Chunk) {
        *self.get_chunk_reference(chunk_position).await.write().await = chunk;
    }
    pub async fn get_block(&self, block_position: BlockPosition) -> Block {
        let (responder, response) = oneshot::channel();
        self.command_sender
            .send(WorldCommand::GetBlock {
                block_position,
                responder,
            })
            .await
            .unwrap();
        response.await.unwrap()
    }
    pub async fn set_block(&self, block_position: BlockPosition, block: Block) {
        self.command_sender
            .send(WorldCommand::SetBlock {
                block_position,
                block,
            })
            .await
            .unwrap();
    }
    pub fn stop(&self) {
        self.running.cancel();
    }
    pub async fn stopped(&self) {
        let _ = self.running.cancelled().await;
    }
}
