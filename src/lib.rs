//! Minecraft Alpha 1.1.2_01 Rust Implementation
//! 
//! A complete cleanroom reimplementation of Minecraft Alpha 1.1.2_01 in Rust,
//! maintaining compatibility with the original game while leveraging modern
//! Rust ecosystem for improved performance and safety.

pub mod engine;
pub mod world;
pub mod render;
pub mod entities;
pub mod blocks;
pub mod audio;
pub mod math;
pub mod nbt;
pub mod camera;
pub mod input;

use anyhow::Result;
use crate::math::position::WorldPos;
use crate::entities::player::Player;

/// Main game interface
pub struct MinecraftAlpha {
    engine: crate::engine::Engine,
    world: crate::world::world_manager::World,
    player: Player,
}

impl MinecraftAlpha {
    /// Create a new game instance
    pub fn new() -> Result<Self> {
        let engine = crate::engine::Engine::new()?;
        let mut world = crate::world::world_manager::World::new(0);
        
        // Spawn player at world origin (0, 64, 0) - above ground level
        let spawn_pos = WorldPos::new(0, 64, 0);
        let player = Player::new(spawn_pos);
        world.add_player(player.clone());
        
        Ok(Self {
            engine,
            world,
            player,
        })
    }
    
    /// Run the main game loop with WebGPU rendering
    pub fn run(&mut self) -> Result<()> {
        println!("Starting Minecraft Alpha 1.1.2_01 - Rust (WebGPU Edition)");
        println!("Features: WebGPU rendering, depth buffer, modular architecture");
        
        // Display player spawn coordinates
        let player_pos = self.player.get_world_pos();
        let chunk_pos = self.player.get_chunk_pos();
        println!("Player spawned at world position: {}", player_pos);
        println!("Player in chunk: {}", chunk_pos);
        
        self.engine.run(self.world.clone())
    }
    
    /// Get reference to the player
    pub fn get_player(&self) -> &Player {
        &self.player
    }
    
    /// Get mutable reference to the player
    pub fn get_player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
}
