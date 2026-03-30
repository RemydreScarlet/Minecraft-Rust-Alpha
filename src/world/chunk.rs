//! Chunk system implementation
//! 
//! This module implements the chunk system equivalent to `ga.java`.

use crate::math::position::{ChunkPos, LocalPos};

/// Chunk data structure
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub blocks: [u8; 32768], // 16x16x128
}

impl Chunk {
    /// Create a new chunk
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z,
            blocks: [0; 32768],
        }
    }
    
    /// Get block at local position
    pub fn get_block(&self, pos: LocalPos) -> u8 {
        self.blocks[pos.to_index()]
    }
    
    /// Set block at local position
    pub fn set_block(&mut self, pos: LocalPos, block_id: u8) -> bool {
        let index = pos.to_index();
        if self.blocks[index] != block_id {
            self.blocks[index] = block_id;
            true
        } else {
            false
        }
    }
}
