//! World storage implementation
//! 
//! This module implements world storage for saving/loading world data.

use std::collections::HashMap;
use crate::math::position::ChunkPos;
use crate::world::chunk::Chunk;

/// World storage manager
pub struct WorldStorage {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl WorldStorage {
    /// Create new world storage
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }
    
    /// Get a chunk from storage
    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }
    
    /// Store a chunk
    pub fn store_chunk(&mut self, chunk: Chunk) {
        let pos = ChunkPos::new(chunk.x, chunk.z);
        self.chunks.insert(pos, chunk);
    }
    
    /// Remove a chunk
    pub fn remove_chunk(&mut self, pos: ChunkPos) -> Option<Chunk> {
        self.chunks.remove(&pos)
    }
}
