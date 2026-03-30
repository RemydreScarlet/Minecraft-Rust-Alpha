//! World generator implementation
//! 
//! This module implements the world generator equivalent to `dn.java`.

use crate::math::position::{ChunkPos, LocalPos};
use crate::world::chunk::Chunk;

/// World generator
pub struct WorldGenerator {
    seed: u64,
}

impl WorldGenerator {
    /// Create a new world generator
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
    
    /// Generate terrain for a chunk
    pub fn generate_chunk(&self, chunk_pos: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new(chunk_pos.x, chunk_pos.z);
        
        // Simple terrain generation - flat grass with some stone
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..128 {
                    let local_pos = LocalPos::new(x, y, z);
                    if y == 0 {
                        chunk.set_block(local_pos, 2); // Grass
                    } else if y < 4 {
                        chunk.set_block(local_pos, 3); // Dirt
                    } else {
                        chunk.set_block(local_pos, 1); // Stone
                    }
                }
            }
        }
        
        chunk
    }
}
