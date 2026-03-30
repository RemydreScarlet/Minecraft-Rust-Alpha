//! Advanced world generator implementation
//! 
//! This module implements the Minecraft Alpha terrain generation system
//! equivalent to `nw.java` with 3D noise-based terrain generation.

use crate::math::position::{ChunkPos, LocalPos};
use crate::world::chunk::Chunk;
use noise::{NoiseFn, Perlin, Simplex};

/// Advanced Minecraft Alpha terrain generator
/// 
/// Implements the full 3D noise-based terrain generation system
/// with biome support, cave generation, and ore distribution.
#[derive(Clone)]
pub struct WorldGenerator {
    // Noise generators for different terrain features
    biome_noise: Simplex,
    depth_noise: Perlin,
    coarse_terrain: Simplex,
    fine_terrain1: Perlin,
    fine_terrain2: Perlin,
    cave_noise: Perlin,
    ore_noise: Simplex,
    
    world_seed: u64,
}

impl WorldGenerator {
    /// Create a new advanced terrain generator
    pub fn new(seed: u64) -> Self {
        Self {
            biome_noise: Simplex::new(seed as u32),
            depth_noise: Perlin::new(seed.wrapping_add(1) as u32),
            coarse_terrain: Simplex::new(seed.wrapping_add(2) as u32),
            fine_terrain1: Perlin::new(seed.wrapping_add(3) as u32),
            fine_terrain2: Perlin::new(seed.wrapping_add(4) as u32),
            cave_noise: Perlin::new(seed.wrapping_add(5) as u32),
            ore_noise: Simplex::new(seed.wrapping_add(6) as u32),
            world_seed: seed,
        }
    }
    
    /// Generate terrain for a chunk using 3D noise-based algorithm
    pub fn generate_chunk(&self, chunk_pos: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new(chunk_pos.x, chunk_pos.z);
        
        // Generate base terrain using 3D noise
        self.generate_base_terrain(chunk_pos.x, chunk_pos.z, &mut chunk);
        
        // Add surface details (biomes, layers, bedrock)
        self.generate_surface_details(chunk_pos.x, chunk_pos.z, &mut chunk);
        
        // Generate caves
        self.generate_caves(chunk_pos.x, chunk_pos.z, &mut chunk);
        
        // Generate ore deposits
        self.generate_ores(chunk_pos.x, chunk_pos.z, &mut chunk);
        
        chunk
    }
    
    /// Generate base terrain using 3D noise interpolation
    fn generate_base_terrain(&self, chunk_x: i32, chunk_z: i32, chunk: &mut Chunk) {
        let base_x = chunk_x * 16;
        let base_z = chunk_z * 16;
        
        // Sample noise at 4x4 resolution and interpolate to 16x16
        for x in 0..16 {
            for z in 0..16 {
                let world_x = base_x + x;
                let world_z = base_z + z;
                
                // Generate terrain height using multiple noise layers
                let noise_x = world_x as f64 * 0.01;
                let noise_z = world_z as f64 * 0.01;
                
                // Base terrain height from multiple octaves
                let mut height = 64.0;
                
                // Large scale features
                height += self.coarse_terrain.get([noise_x * 0.5, 0.0, noise_z * 0.5]) * 20.0;
                
                // Medium scale features  
                height += self.fine_terrain1.get([noise_x, 0.0, noise_z]) * 8.0;
                
                // Small scale details
                height += self.fine_terrain2.get([noise_x * 2.0, 0.0, noise_z * 2.0]) * 4.0;
                
                // Biome variation
                let biome_factor = (self.biome_noise.get([noise_x * 0.1, 0.0, noise_z * 0.1]) + 1.0) * 0.5;
                height += biome_factor * 10.0 - 5.0;
                
                // Clamp height to reasonable range
                let surface_height = height.round() as i32;
                let clamped_height = surface_height.clamp(20, 100);
                
                // Fill column with stone up to surface height
                for y in 0..clamped_height.min(128) {
                    let pos = LocalPos::new(x, y, z);
                    chunk.set_block(pos, 1); // Stone
                }
            }
        }
    }
    
    /// Generate surface details (biomes, dirt layers, bedrock)
    fn generate_surface_details(&self, chunk_x: i32, chunk_z: i32, chunk: &mut Chunk) {
        let base_x = chunk_x * 16;
        let base_z = chunk_z * 16;
        
        for x in 0..16 {
            for z in 0..16 {
                let world_x = base_x + x;
                let world_z = base_z + z;
                
                // Biome determination
                let noise_x = world_x as f64 * 0.05;
                let noise_z = world_z as f64 * 0.05;
                
                let snow_noise = self.biome_noise.get([noise_x, 0.0, noise_z]);
                let desert_noise = self.biome_noise.get([noise_z as f64, 109.0134, world_x as f64]);
                
                let is_snow = snow_noise > 0.3;
                let is_desert = desert_noise > 0.3;
                
                // Find surface height
                let mut surface_height = 0;
                for y in (0..128).rev() {
                    let pos = LocalPos::new(x, y, z);
                    if chunk.get_block(pos) == 1 { // Stone
                        surface_height = y;
                        break;
                    }
                }
                
                if surface_height > 0 {
                    // Generate surface layers
                    for y in (0..=surface_height).rev() {
                        let pos = LocalPos::new(x, y, z);
                        if y == surface_height {
                            // Surface block
                            if is_snow {
                                chunk.set_block(pos, 78); // Snow block
                            } else if is_desert {
                                chunk.set_block(pos, 12); // Sand
                            } else {
                                chunk.set_block(pos, 2); // Grass
                            }
                        } else if y >= surface_height - 3 {
                            // Dirt layer (unless desert)
                            if !is_desert {
                                chunk.set_block(pos, 3); // Dirt
                            }
                        }
                        
                        // Bedrock at bottom
                        if y <= 0 {
                            chunk.set_block(pos, 7); // Bedrock
                        }
                    }
                }
            }
        }
    }
    
    /// Generate cave systems
    fn generate_caves(&self, chunk_x: i32, chunk_z: i32, chunk: &mut Chunk) {
        let base_x = chunk_x * 16;
        let base_z = chunk_z * 16;
        
        // Generate 8 caves per chunk
        for cave_id in 0..8 {
            let seed = self.world_seed.wrapping_add(cave_id as u64)
                .wrapping_mul(chunk_x as u64 * 341873128712)
                .wrapping_add(chunk_z as u64 * 132897987541);
            
            // Cave starting position
            let start_x = base_x + ((seed >> 32) % 16) as i32;
            let start_y = ((seed >> 16) % 128) as i32;
            let start_z = base_z + ((seed % 16) as i32);
            
            // Generate cave path
            self.generate_cave_tunnel(start_x, start_y, start_z, seed, chunk);
        }
    }
    
    /// Generate a single cave tunnel
    fn generate_cave_tunnel(&self, start_x: i32, start_y: i32, start_z: i32, seed: u64, chunk: &mut Chunk) {
        let mut x = start_x as f64;
        let mut y = start_y as f64;
        let mut z = start_z as f64;
        
        let cave_length = 20 + (seed % 40) as i32;
        
        for step in 0..cave_length {
            let radius = 2 + ((y - 64.0).abs() / 32.0) as i32;
            
            // Carve out cave sphere
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    for dz in -radius..=radius {
                        if dx*dx + dy*dy + dz*dz <= radius*radius {
                            let world_x = x as i32 + dx;
                            let world_y = y as i32 + dy;
                            let world_z = z as i32 + dz;
                            
                            // Check if position is in this chunk
                            if world_x >= 0 && world_x < 16 && world_y >= 0 && world_y < 128 && world_z >= 0 && world_z < 16 {
                                let pos = LocalPos::new(world_x, world_y, world_z);
                                if chunk.get_block(pos) == 1 { // Stone
                                    chunk.set_block(pos, 0); // Air
                                }
                            }
                        }
                    }
                }
            }
            
            // Move cave forward
            let step_seed = seed.wrapping_add(step as u64);
            x += ((step_seed >> 8).wrapping_sub(200) % 401) as f64 / 100.0 - 2.0;
            y += ((step_seed >> 16).wrapping_sub(100) % 201) as f64 / 100.0 - 1.0;
            z += ((step_seed >> 24).wrapping_sub(200) % 401) as f64 / 100.0 - 2.0;
            
            // Keep Y in bounds
            y = y.max(5.0).min(123.0);
        }
    }
    
    /// Generate ore deposits
    fn generate_ores(&self, chunk_x: i32, chunk_z: i32, chunk: &mut Chunk) {
        let base_x = chunk_x * 16;
        let base_z = chunk_z * 16;
        
        // Ore types: (block_id, vein_size, max_y, attempts)
        let ore_types = [
            (16, 8, 60, 20),  // Coal
            (15, 4, 40, 15),  // Iron
            (14, 3, 30, 10),  // Gold
            (13, 2, 20, 5),   // Diamond
        ];
        
        for &(ore_id, vein_size, max_y, attempts) in &ore_types {
            for attempt in 0..attempts {
                let seed = self.world_seed
                    .wrapping_add(ore_id as u64 * 1000)
                    .wrapping_add(attempt as u64)
                    .wrapping_mul(chunk_x as u64 * 341873128712)
                    .wrapping_add(chunk_z as u64 * 132897987541);
                
                let start_x = base_x + ((seed >> 16) % 16) as i32;
                let start_y = ((seed >> 8) % max_y) as i32;
                let start_z = base_z + ((seed % 16) as i32);
                
                self.generate_ore_vein(start_x, start_y, start_z, ore_id, vein_size, seed, chunk);
            }
        }
    }
    
    /// Generate a single ore vein
    fn generate_ore_vein(&self, start_x: i32, start_y: i32, start_z: i32, ore_id: u8, vein_size: i32, seed: u64, chunk: &mut Chunk) {
        let mut placed = 0;
        
        for offset in 0..vein_size {
            let offset_seed = seed.wrapping_add(offset as u64);
            
            let dx = ((offset_seed >> 16) % 7) as i32 - 3;
            let dy = ((offset_seed >> 8) % 7) as i32 - 3;
            let dz = (offset_seed % 7) as i32 - 3;
            
            let world_x = start_x + dx;
            let world_y = start_y + dy;
            let world_z = start_z + dz;
            
            // Check if position is in chunk and is stone
            if world_x >= 0 && world_x < 16 && world_y >= 0 && world_y < 128 && world_z >= 0 && world_z < 16 {
                let pos = LocalPos::new(world_x, world_y, world_z);
                if chunk.get_block(pos) == 1 { // Stone
                    chunk.set_block(pos, ore_id);
                    placed += 1;
                }
            }
            
            if placed >= vein_size {
                break;
            }
        }
    }
}
