//! World generator implementation
//! 
//! This module implements the world generator equivalent to `dn.java`.

use crate::math::position::{ChunkPos, LocalPos};
use crate::world::chunk::Chunk;

/// World generator
#[derive(Clone)]
pub struct WorldGenerator {
    seed: u64,
}

impl WorldGenerator {
    /// Create a new world generator
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
    
    /// Generate terrain for a chunk with random heightmap
    pub fn generate_chunk(&self, chunk_pos: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new(chunk_pos.x, chunk_pos.z);
        
        // Generate heightmap for this chunk
        let mut heightmap = [[0; 16]; 16];
        
        for x in 0..16 {
            for z in 0..16 {
                // Use simple hash-based height generation
                let world_x = chunk_pos.x * 16 + x;
                let world_z = chunk_pos.z * 16 + z;
                heightmap[x as usize][z as usize] = self.generate_height(world_x, world_z);
            }
        }
        
        // Fill chunk with blocks based on heightmap
        for x in 0..16 {
            for z in 0..16 {
                let surface_height = heightmap[x as usize][z as usize];
                
                for y in 0..128 {
                    let local_pos = LocalPos::new(x, y, z);
                    
                    if y > surface_height {
                        // Above surface - air blocks (skip, already air)
                        continue;
                    } else if y == surface_height {
                        // Surface layer - grass
                        chunk.set_block(local_pos, 2);
                    } else if y >= surface_height - 3 {
                        // Dirt layer below grass
                        chunk.set_block(local_pos, 3);
                    } else if y >= 20 {
                        // Stone layer
                        chunk.set_block(local_pos, 1);
                    } else {
                        // Deep underground - mostly stone with some air pockets
                        if self.should_be_air(x, y, z, chunk_pos.x, chunk_pos.z) {
                            // Air pocket/cave
                            continue;
                        } else {
                            chunk.set_block(local_pos, 1);
                        }
                    }
                }
            }
        }
        
        chunk
    }
    
    /// Generate height for a specific world position
    fn generate_height(&self, world_x: i32, world_z: i32) -> i32 {
        // Simple hash-based pseudo-random height generation
        let hash = self.hash_coords(world_x, world_z);
        let base_height = 50;
        let variation = 30;
        
        // Height between 20 and 80
        base_height + (hash % variation) - (variation / 2)
    }
    
    /// Check if a position should be air (for caves)
    fn should_be_air(&self, local_x: i32, y: i32, local_z: i32, chunk_x: i32, chunk_z: i32) -> bool {
        // Simple cave generation - 10% chance of air pockets below y=40
        if y < 40 {
            let world_x = chunk_x * 16 + local_x;
            let world_z = chunk_z * 16 + local_z;
            let hash = self.hash_coords_3d(world_x, y, world_z);
            hash % 10 == 0
        } else {
            false
        }
    }
    
    /// Simple hash function for 2D coordinates
    fn hash_coords(&self, x: i32, z: i32) -> i32 {
        let mut hash = self.seed as i32;
        hash = hash.wrapping_mul(31).wrapping_add(x);
        hash = hash.wrapping_mul(31).wrapping_add(z);
        hash.wrapping_abs()
    }
    
    /// Simple hash function for 3D coordinates
    fn hash_coords_3d(&self, x: i32, y: i32, z: i32) -> i32 {
        let mut hash = self.seed as i32;
        hash = hash.wrapping_mul(31).wrapping_add(x);
        hash = hash.wrapping_mul(31).wrapping_add(y);
        hash = hash.wrapping_mul(31).wrapping_add(z);
        hash.wrapping_abs()
    }
}
