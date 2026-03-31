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
    
    /// Chunk loading configuration
    render_distance: i32,  // Chunk render distance
    max_loaded_chunks: usize,  // Maximum chunks to keep in memory
    
    /// Terrain generation system
    chunk_generation_queue: Vec<ChunkPos>,  // Chunks waiting to be generated
    chunks_per_tick: usize,  // How many chunks to generate per tick
    
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
            render_distance: 4,  // 4 chunks render distance (good balance)
            max_loaded_chunks: 81,   // Maximum 81 chunks in memory (9x9 area)
            chunk_generation_queue: Vec::new(),
            chunks_per_tick: 2,  // Generate 2 chunks per tick for smooth performance
        };
        
        // Generate initial chunks around spawn
        world.generate_initial_chunks();
        world
    }
    
    /// Generate initial chunks around spawn
    fn generate_initial_chunks(&mut self) {
        let center_chunk = ChunkPos::new(self.spawn_x >> 4, self.spawn_z >> 4);
        
        // Generate only the center chunk and immediate neighbors first (9 chunks total)
        for dx in -1..=1 {
            for dz in -1..=1 {
                let chunk_pos = ChunkPos::new(center_chunk.x + dx, center_chunk.z + dz);
                if !self.chunks.contains_key(&chunk_pos) {
                    let chunk = self.generator.generate_chunk(chunk_pos);
                    self.add_chunk(chunk);
                }
            }
        }
        
        println!("DEBUG: Generated initial 9 chunks around spawn");
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
        
        // Process chunk generation queue (tick-based terrain generation)
        self.process_chunk_generation_queue();
        
        // Update entities
        for entity in &mut self.entities {
            entity.update();
        }
        
        // Update players
        for player in &mut self.players {
            player.update();
        }
        
        // Update chunk loading based on player position
        self.update_chunk_loading();
    }
    
    /// Process chunk generation queue (tick-based terrain generation)
    fn process_chunk_generation_queue(&mut self) {
        let mut generated_count = 0;
        
        // Process up to chunks_per_tick chunks from the queue
        while generated_count < self.chunks_per_tick && !self.chunk_generation_queue.is_empty() {
            if let Some(chunk_pos) = self.chunk_generation_queue.pop() {
                if !self.chunks.contains_key(&chunk_pos) {
                    let chunk = self.generator.generate_chunk(chunk_pos);
                    self.add_chunk(chunk);
                    generated_count += 1;
                }
            }
        }
        
        if generated_count > 0 {
            println!("DEBUG: Generated {} chunks this tick, {} remaining in queue", 
                     generated_count, self.chunk_generation_queue.len());
        }
    }
    
    /// Update chunk loading based on player positions
    fn update_chunk_loading(&mut self) {
        if let Some(player) = self.get_player() {
            let player_chunk = ChunkPos::new(
                (player.entity.x as i32) >> 4, 
                (player.entity.z as i32) >> 4
            );
            self.load_chunks_around_player(player_chunk);
            self.unload_distant_chunks(player_chunk);
        }
    }
    
    /// Load chunks around the player position
    fn load_chunks_around_player(&mut self, center_chunk: ChunkPos) {
        for dx in -self.render_distance..=self.render_distance {
            for dz in -self.render_distance..=self.render_distance {
                let chunk_pos = ChunkPos::new(center_chunk.x + dx, center_chunk.z + dz);
                
                // Check if chunk is within render distance (circular distance)
                let distance_sq = dx * dx + dz * dz;
                if distance_sq <= self.render_distance * self.render_distance {
                    if !self.chunks.contains_key(&chunk_pos) && 
                       !self.chunk_generation_queue.contains(&chunk_pos) {
                        // Add to generation queue instead of generating immediately
                        self.chunk_generation_queue.push(chunk_pos);
                    }
                }
            }
        }
    }
    
    /// Unload chunks that are too far from the player
    fn unload_distant_chunks(&mut self, center_chunk: ChunkPos) {
        let mut chunks_to_remove = Vec::new();
        
        for (&chunk_pos, _) in &self.chunks {
            let dx = chunk_pos.x - center_chunk.x;
            let dz = chunk_pos.z - center_chunk.z;
            let distance_sq = dx * dx + dz * dz;
            
            // Unload if outside render distance or if we have too many chunks
            let max_distance = self.render_distance + 2;  // Keep some buffer
            if distance_sq > max_distance * max_distance || 
               self.chunks.len() > self.max_loaded_chunks {
                chunks_to_remove.push(chunk_pos);
            }
        }
        
        // Remove distant chunks
        for chunk_pos in chunks_to_remove {
            self.chunks.remove(&chunk_pos);
        }
    }
    
    /// Get chunks within render distance of a position
    pub fn get_chunks_near(&self, center_chunk: ChunkPos) -> Vec<&Chunk> {
        let mut nearby_chunks = Vec::new();
        
        for dx in -self.render_distance..=self.render_distance {
            for dz in -self.render_distance..=self.render_distance {
                let chunk_pos = ChunkPos::new(center_chunk.x + dx, center_chunk.z + dz);
                let distance_sq = dx * dx + dz * dz;
                
                if distance_sq <= self.render_distance * self.render_distance {
                    if let Some(chunk) = self.chunks.get(&chunk_pos) {
                        nearby_chunks.push(chunk);
                    }
                }
            }
        }
        
        nearby_chunks
    }
    
    /// Check if a chunk is loaded
    pub fn is_chunk_loaded(&self, pos: ChunkPos) -> bool {
        self.chunks.contains_key(&pos)
    }
    
    /// Force load a specific chunk
    pub fn load_chunk(&mut self, pos: ChunkPos) -> bool {
        if !self.chunks.contains_key(&pos) {
            let chunk = self.generator.generate_chunk(pos);
            self.add_chunk(chunk);
            true
        } else {
            false
        }
    }
    
    /// Get the first player (for single player mode)
    pub fn get_player(&self) -> Option<&Player> {
        self.players.first()
    }
    
    /// Get the first player as mutable (for single player mode)
    pub fn get_player_mut(&mut self) -> Option<&mut Player> {
        self.players.first_mut()
    }
    
    /// Add a player to the world
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}
