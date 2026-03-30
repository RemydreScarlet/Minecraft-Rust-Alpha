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

use anyhow::Result;

/// Main game interface
pub struct MinecraftAlpha {
    engine: crate::engine::Engine,
    world: crate::world::world::World,
    renderer: crate::render::Renderer,
}

impl MinecraftAlpha {
    /// Create a new game instance
    pub fn new() -> Result<Self> {
        env_logger::init();
        
        let engine = crate::engine::Engine::new()?;
        let world = crate::world::world::World::new(0);
        let renderer = crate::render::Renderer::new()?;
        
        Ok(Self {
            engine,
            world,
            renderer,
        })
    }
    
    /// Run the main game loop
    pub fn run(&mut self) -> Result<()> {
        self.engine.run(&mut self.world, &mut self.renderer)
    }
}
