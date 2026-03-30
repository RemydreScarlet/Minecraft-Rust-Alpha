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

/// Main game interface
pub struct MinecraftAlpha {
    engine: crate::engine::Engine,
    world: crate::world::world_manager::World,
}

impl MinecraftAlpha {
    /// Create a new game instance
    pub fn new() -> Result<Self> {
        let engine = crate::engine::Engine::new()?;
        let world = crate::world::world_manager::World::new(0);
        
        Ok(Self {
            engine,
            world,
        })
    }
    
    /// Run the main game loop with WebGPU rendering
    pub fn run(&mut self) -> Result<()> {
        println!("Starting Minecraft Alpha 1.1.2_01 - Rust (WebGPU Edition)");
        println!("Features: WebGPU rendering, depth buffer, modular architecture");
        
        self.engine.run(self.world.clone())
    }
}
