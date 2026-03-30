//! Chunk renderer implementation
//! 
//! This module implements chunk rendering equivalent to `bn.java`.

use anyhow::Result;
use wgpu::{util::DeviceExt, Buffer, Device};
use crate::world::chunk::Chunk;

/// Vertex structure for 3D rendering
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ChunkVertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl ChunkVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ChunkVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

/// Chunk renderer
pub struct ChunkRenderer {
    vertex_buffer: Option<Buffer>,
    index_buffer: Option<Buffer>,
    num_indices: u32,
}

impl ChunkRenderer {
    /// Create a new chunk renderer
    pub fn new() -> Result<Self> {
        Ok(Self {
            vertex_buffer: None,
            index_buffer: None,
            num_indices: 0,
        })
    }
    
    /// Generate mesh for a chunk
    pub fn generate_chunk_mesh(&mut self, chunk: &Chunk, device: &Device) -> Result<()> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        // Generate cubes for each non-air block
        for y in 0..128 {
            for x in 0..16 {
                for z in 0..16 {
                    let local_pos = crate::math::position::LocalPos::new(x, y, z);
                    let block_id = chunk.get_block(local_pos);
                    
                    if block_id != 0 {  // Not air
                        let world_x = chunk.x * 16 + x;
                        let world_y = y;
                        let world_z = chunk.z * 16 + z;
                        
                        self.add_cube_vertices(
                            world_x as f32, 
                            world_y as f32, 
                            world_z as f32, 
                            block_id,
                            &mut vertices,
                            &mut indices,
                        );
                    }
                }
            }
        }
        
        if !vertices.is_empty() {
            self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }));
            
            self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }));
            
            self.num_indices = indices.len() as u32;
        }
        
        Ok(())
    }
    
    /// Add cube vertices for a block
    fn add_cube_vertices(&self, x: f32, y: f32, z: f32, block_id: u8, 
                        vertices: &mut Vec<ChunkVertex>, indices: &mut Vec<u16>) {
        let color = self.get_block_color(block_id);
        let base_index = vertices.len() as u16;
        
        // Define cube vertices
        let cube_vertices = [
            // Front face
            ChunkVertex { position: [x, y, z + 1.0], color },
            ChunkVertex { position: [x + 1.0, y, z + 1.0], color },
            ChunkVertex { position: [x + 1.0, y + 1.0, z + 1.0], color },
            ChunkVertex { position: [x, y + 1.0, z + 1.0], color },
            // Back face
            ChunkVertex { position: [x, y, z], color },
            ChunkVertex { position: [x + 1.0, y, z], color },
            ChunkVertex { position: [x + 1.0, y + 1.0, z], color },
            ChunkVertex { position: [x, y + 1.0, z], color },
            // Top face
            ChunkVertex { position: [x, y + 1.0, z], color },
            ChunkVertex { position: [x + 1.0, y + 1.0, z], color },
            ChunkVertex { position: [x + 1.0, y + 1.0, z + 1.0], color },
            ChunkVertex { position: [x, y + 1.0, z + 1.0], color },
            // Bottom face
            ChunkVertex { position: [x, y, z], color },
            ChunkVertex { position: [x + 1.0, y, z], color },
            ChunkVertex { position: [x + 1.0, y, z + 1.0], color },
            ChunkVertex { position: [x, y, z + 1.0], color },
            // Right face
            ChunkVertex { position: [x + 1.0, y, z], color },
            ChunkVertex { position: [x + 1.0, y, z + 1.0], color },
            ChunkVertex { position: [x + 1.0, y + 1.0, z + 1.0], color },
            ChunkVertex { position: [x + 1.0, y + 1.0, z], color },
            // Left face
            ChunkVertex { position: [x, y, z], color },
            ChunkVertex { position: [x, y, z + 1.0], color },
            ChunkVertex { position: [x, y + 1.0, z + 1.0], color },
            ChunkVertex { position: [x, y + 1.0, z], color },
        ];
        
        vertices.extend_from_slice(&cube_vertices);
        
        // Define cube indices
        let cube_indices = [
            0, 1, 2, 2, 3, 0,  // front
            4, 6, 5, 6, 4, 7,  // back
            8, 9, 10, 10, 11, 8,  // top
            12, 14, 13, 14, 12, 15,  // bottom
            16, 17, 18, 18, 19, 16,  // right
            20, 22, 21, 22, 20, 23,  // left
        ];
        
        for &index in &cube_indices {
            indices.push(base_index + index);
        }
    }
    
    /// Get color for a block type
    fn get_block_color(&self, block_id: u8) -> [f32; 3] {
        match block_id {
            1 => [0.5, 0.5, 0.5],  // Stone - gray
            2 => [0.2, 0.5, 0.2],  // Grass - green
            3 => [0.4, 0.3, 0.2],  // Dirt - brown
            4 => [0.6, 0.6, 0.6],  // Cobblestone - light gray
            5 => [0.5, 0.3, 0.1],  // Wood - brown
            _ => [0.8, 0.2, 0.8],  // Default - purple
        }
    }
    
    /// Get the vertex buffer
    pub fn vertex_buffer(&self) -> Option<&Buffer> {
        self.vertex_buffer.as_ref()
    }
    
    /// Get the index buffer
    pub fn index_buffer(&self) -> Option<&Buffer> {
        self.index_buffer.as_ref()
    }
    
    /// Get the number of indices
    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }
}
