//! Minecraft Alpha 1.1.2_01 - Main Entry Point

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    
    let mut game = minecraft_alpha_rust::MinecraftAlpha::new()?;
    game.run()
}
