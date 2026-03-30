//! World management
//! 
//! This module implements the main world class that manages all game state,
//! chunks, entities, and world data. Equivalent to `cn.java` in the original.

use crate::math::position::{WorldPos, ChunkPos};
use crate::world::chunk::Chunk;
use crate::world::generator::WorldGenerator;
use crate::entities::entity::Entity;
use crate::entities::player::Player;
use std::collections::HashMap;

/// Main world class that manages all game state, chunks, entities, and world data
/// 
/// This is equivalent to the `cn` class in the original Java code.
#[derive(Clone)]
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
    
    /// Terrain generator
    generator: WorldGenerator,
}

impl World {
    /// Create a new world with the given seed
    pub fn new(seed: u64) -> Self {
        let generator = WorldGenerator::new(seed);
        let mut world = Self {
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
            generator,
        };
        
        // Generate initial chunk at spawn
        world.generate_initial_chunks();
        world
    }
    
    /// Generate initial chunks around spawn
    fn generate_initial_chunks(&mut self) {
        // Generate a single chunk at (0,0) for now
        let chunk_pos = ChunkPos::new(0, 0);
        let chunk = self.generator.generate_chunk(chunk_pos);
        self.add_chunk(chunk);
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
    
    /// Get all chunks for rendering
    pub fn get_all_chunks(&self) -> Vec<&Chunk> {
        self.chunks.values().collect()
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
