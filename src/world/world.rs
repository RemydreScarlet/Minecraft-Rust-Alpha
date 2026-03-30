//! World management
//! 
//! This module implements the main world class that manages all game state,
//! chunks, entities, and world data. Equivalent to `cn.java` in the original.

use crate::math::position::{WorldPos, ChunkPos};
use std::collections::HashMap;

/// Main world class that manages all game state, chunks, entities, and world data
/// 
/// This is equivalent to the `cn` class in the original Java code.
pub struct World {
    /// World properties
    pub seed: u64,
    pub time: u64,
    pub spawn_x: i32,
    pub spawn_y: i32,
    pub spawn_z: i32,
    pub dimension: i32, // 0 = overworld
    
    /// Chunk management
    chunks: HashMap<ChunkPos, Chunk>,
    
    /// Entity management
    entities: Vec<Entity>,
    players: Vec<Player>,
    
    /// World flags
    pub snow_covered: bool,
    pub multiplayer: bool,
}

impl World {
    /// Create a new world with the given seed
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            time: 0,
            spawn_x: 0,
            spawn_y: 64, // Default spawn height
            spawn_z: 0,
            dimension: 0,
            chunks: HashMap::new(),
            entities: Vec::new(),
            players: Vec::new(),
            snow_covered: false,
            multiplayer: false,
        }
    }
    
    /// Get the block type at the given world coordinates
    pub fn get_block(&self, pos: WorldPos) -> u8 {
        let chunk_pos = pos.to_chunk_pos();
        let local_pos = pos.to_local_pos();
        
        if let Some(chunk) = self.chunks.get(&chunk_pos) {
            chunk.get_block(local_pos)
        } else {
            0 // Air block
        }
    }
    
    /// Set a block at the given world coordinates
    pub fn set_block(&mut self, pos: WorldPos, block_id: u8) -> bool {
        let chunk_pos = pos.to_chunk_pos();
        let local_pos = pos.to_local_pos();
        
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.set_block(local_pos, block_id)
        } else {
            false
        }
    }
    
    /// Get a chunk at the given chunk coordinates
    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }
    
    /// Get a mutable chunk at the given chunk coordinates
    pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
        self.chunks.get_mut(&pos)
    }
    
    /// Add a chunk to the world
    pub fn add_chunk(&mut self, chunk: Chunk) {
        let pos = ChunkPos::new(chunk.x, chunk.z);
        self.chunks.insert(pos, chunk);
    }
    
    /// Remove a chunk from the world
    pub fn remove_chunk(&mut self, pos: ChunkPos) -> Option<Chunk> {
        self.chunks.remove(&pos)
    }
    
    /// Update world state (called each game tick)
    pub fn update(&mut self) {
        self.time += 1;
        
        // Update entities
        for entity in &mut self.entities {
            entity.update();
        }
        
        // Update players
        for player in &mut self.players {
            player.update();
        }
    }
}

// Forward declarations for now
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub blocks: [u8; 32768], // 16x16x128
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z,
            blocks: [0; 32768],
        }
    }
    
    pub fn get_block(&self, pos: crate::math::position::LocalPos) -> u8 {
        self.blocks[pos.to_index()]
    }
    
    pub fn set_block(&mut self, pos: crate::math::position::LocalPos, block_id: u8) -> bool {
        let index = pos.to_index();
        if self.blocks[index] != block_id {
            self.blocks[index] = block_id;
            true
        } else {
            false
        }
    }
}

pub struct Entity {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Entity {
    pub fn update(&mut self) {
        // Basic entity update logic
    }
}

pub struct Player {
    pub entity: Entity,
    pub health: i32,
}

impl Player {
    pub fn update(&mut self) {
        self.entity.update();
    }
}
