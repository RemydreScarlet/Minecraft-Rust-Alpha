//! Minecraft Alpha 1.1.2_01 - Main Entry Point
//! 
//! This is the main entry point for the Minecraft Alpha Rust implementation.

use minecraft_alpha_rust::MinecraftAlpha;
use anyhow::Result;

fn main() -> Result<()> {
    let mut game = MinecraftAlpha::new()?;
    game.run()
}
