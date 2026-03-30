//! World coordinates system
//! 
//! This module implements coordinate conversions and utilities.

use crate::math::position::{WorldPos, ChunkPos, LocalPos};

/// Coordinate conversion utilities
pub struct CoordinateUtils;

impl CoordinateUtils {
    /// Convert world coordinates to chunk coordinates
    pub fn world_to_chunk(world_pos: WorldPos) -> ChunkPos {
        ChunkPos::new(
            world_pos.x >> 4,
            world_pos.z >> 4
        )
    }
    
    /// Convert chunk coordinates to world coordinates
    pub fn chunk_to_world(chunk_pos: ChunkPos) -> WorldPos {
        WorldPos::new(
            (chunk_pos.x << 4) as i32,
            0, // Default Y
            (chunk_pos.z << 4) as i32
        )
    }
}
