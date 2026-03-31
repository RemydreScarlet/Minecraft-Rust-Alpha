//! Player entity implementation
//! 
//! This module implements the player entity with world coordinate support.

use crate::entities::entity::Entity;
use crate::math::position::{WorldPos, ChunkPos, LocalPos};

#[derive(Clone)]
pub struct Player {
    pub entity: Entity,
    pub health: i32,
}

impl Player {
    /// Create a new player at the specified world position
    pub fn new(world_pos: WorldPos) -> Self {
        Self {
            entity: Entity {
                id: 0, // Player typically has ID 0
                x: world_pos.x as f64,
                y: world_pos.y as f64,
                z: world_pos.z as f64,
            },
            health: 20, // Default max health
        }
    }
    
    /// Get the player's current world position (float precision)
    pub fn get_position(&self) -> (f64, f64, f64) {
        (self.entity.x, self.entity.y, self.entity.z)
    }
    
    /// Set the player's world position (float precision)
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.entity.x = x;
        self.entity.y = y;
        self.entity.z = z;
    }
    
    /// Get the player's current world position (integer for block operations)
    pub fn get_world_pos(&self) -> WorldPos {
        WorldPos::new(
            self.entity.x as i32,
            self.entity.y as i32,
            self.entity.z as i32,
        )
    }
    
    /// Set the player's world position (integer input)
    pub fn set_world_pos(&mut self, world_pos: WorldPos) {
        self.entity.x = world_pos.x as f64;
        self.entity.y = world_pos.y as f64;
        self.entity.z = world_pos.z as f64;
    }
    
    /// Get the player's current chunk position
    pub fn get_chunk_pos(&self) -> ChunkPos {
        self.get_world_pos().to_chunk_pos()
    }
    
    /// Get the player's current local position within their chunk
    pub fn get_local_pos(&self) -> LocalPos {
        self.get_world_pos().to_local_pos()
    }
    
    /// Move player by relative world coordinates
    pub fn move_by(&mut self, dx: i32, dy: i32, dz: i32) {
        let current_pos = self.get_world_pos();
        let new_pos = WorldPos::new(
            current_pos.x + dx,
            current_pos.y + dy,
            current_pos.z + dz,
        );
        self.set_world_pos(new_pos);
    }
    
    /// Update player state
    pub fn update(&mut self) {
        self.entity.update();
    }
}
