//! Chunk mesh generation system
//! 
//! This module handles the generation of triangle meshes for chunk rendering,
//! including face culling and vertex/index buffer creation.

use wgpu::{util::DeviceExt, Device, Buffer};

/// Vertex structure for block rendering
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct BlockVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl BlockVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BlockVertex>() as wgpu::BufferAddress,
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

/// Block face directions
#[derive(Debug, Clone, Copy)]
pub enum FaceDirection {
    PositiveX, // Right face
    NegativeX, // Left face
    PositiveY, // Top face
    NegativeY, // Bottom face
    PositiveZ, // Front face
    NegativeZ, // Back face
}

/// Chunk mesh data
pub struct ChunkMesh {
    vertices: Vec<BlockVertex>,
    indices: Vec<u32>,
}

impl ChunkMesh {
    /// Create a new empty chunk mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Add a block face to the mesh
    pub fn add_face(&mut self, x: i32, y: i32, z: i32, direction: FaceDirection, block_type: u8) {
        let color = self.get_block_color(block_type);
        let base_x = x as f32;
        let base_y = y as f32;
        let base_z = z as f32;

        let (face_vertices, face_indices) = match direction {
            FaceDirection::PositiveX => {
                // Right face (X+) - Red
                let vertices = vec![
                    BlockVertex { position: [base_x + 1.0, base_y, base_z + 1.0], color },
                    BlockVertex { position: [base_x + 1.0, base_y, base_z], color },
                    BlockVertex { position: [base_x + 1.0, base_y + 1.0, base_z], color },
                    BlockVertex { position: [base_x + 1.0, base_y + 1.0, base_z + 1.0], color },
                ];
                let indices = vec![0, 1, 2, 2, 3, 0];
                (vertices, indices)
            },
            FaceDirection::NegativeX => {
                // Left face (X-) - Cyan
                let vertices = vec![
                    BlockVertex { position: [base_x, base_y, base_z], color },
                    BlockVertex { position: [base_x, base_y, base_z + 1.0], color },
                    BlockVertex { position: [base_x, base_y + 1.0, base_z + 1.0], color },
                    BlockVertex { position: [base_x, base_y + 1.0, base_z], color },
                ];
                let indices = vec![0, 1, 2, 2, 3, 0];
                (vertices, indices)
            },
            FaceDirection::PositiveY => {
                // Top face (Y+) - Blue
                let vertices = vec![
                    BlockVertex { position: [base_x, base_y + 1.0, base_z + 1.0], color },
                    BlockVertex { position: [base_x + 1.0, base_y + 1.0, base_z + 1.0], color },
                    BlockVertex { position: [base_x + 1.0, base_y + 1.0, base_z], color },
                    BlockVertex { position: [base_x, base_y + 1.0, base_z], color },
                ];
                let indices = vec![0, 1, 2, 2, 3, 0];
                (vertices, indices)
            },
            FaceDirection::NegativeY => {
                // Bottom face (Y-) - Yellow
                let vertices = vec![
                    BlockVertex { position: [base_x, base_y, base_z], color },
                    BlockVertex { position: [base_x + 1.0, base_y, base_z], color },
                    BlockVertex { position: [base_x + 1.0, base_y, base_z + 1.0], color },
                    BlockVertex { position: [base_x, base_y, base_z + 1.0], color },
                ];
                let indices = vec![0, 1, 2, 2, 3, 0];
                (vertices, indices)
            },
            FaceDirection::PositiveZ => {
                // Front face (Z+) - Red
                let vertices = vec![
                    BlockVertex { position: [base_x, base_y, base_z + 1.0], color },
                    BlockVertex { position: [base_x + 1.0, base_y, base_z + 1.0], color },
                    BlockVertex { position: [base_x + 1.0, base_y + 1.0, base_z + 1.0], color },
                    BlockVertex { position: [base_x, base_y + 1.0, base_z + 1.0], color },
                ];
                let indices = vec![0, 1, 2, 2, 3, 0];
                (vertices, indices)
            },
            FaceDirection::NegativeZ => {
                // Back face (Z-) - Green
                let vertices = vec![
                    BlockVertex { position: [base_x + 1.0, base_y, base_z], color },
                    BlockVertex { position: [base_x, base_y, base_z], color },
                    BlockVertex { position: [base_x, base_y + 1.0, base_z], color },
                    BlockVertex { position: [base_x + 1.0, base_y + 1.0, base_z], color },
                ];
                let indices = vec![0, 1, 2, 2, 3, 0];
                (vertices, indices)
            },
        };

        // Add vertices and update indices
        let vertex_offset = self.vertices.len() as u32;
        self.vertices.extend(face_vertices);
        self.indices.extend(face_indices.iter().map(|&i| i + vertex_offset));
    }

    /// Generate mesh for a chunk with face culling
    pub fn generate_chunk_mesh(chunk: &crate::world::chunk::Chunk, world: &crate::world::world_manager::World) -> Self {
        let mut mesh = Self::new();
        let mut solid_blocks = 0;
        let mut faces_rendered = 0;

        for x in 0..16 {
            for y in 0..128 {
                for z in 0..16 {
                    let local_pos = crate::math::position::LocalPos::new(x, y, z);
                    let block_type = chunk.get_block(local_pos);

                    if block_type == 0 {
                        continue; // Skip air blocks
                    }
                    
                    solid_blocks += 1;

                    // Check each face for visibility
                    if Self::should_render_face(chunk, world, x, y, z, FaceDirection::PositiveX) {
                        mesh.add_face(x, y, z, FaceDirection::PositiveX, block_type);
                        faces_rendered += 1;
                    }
                    if Self::should_render_face(chunk, world, x, y, z, FaceDirection::NegativeX) {
                        mesh.add_face(x, y, z, FaceDirection::NegativeX, block_type);
                        faces_rendered += 1;
                    }
                    if Self::should_render_face(chunk, world, x, y, z, FaceDirection::PositiveY) {
                        mesh.add_face(x, y, z, FaceDirection::PositiveY, block_type);
                        faces_rendered += 1;
                    }
                    if Self::should_render_face(chunk, world, x, y, z, FaceDirection::NegativeY) {
                        mesh.add_face(x, y, z, FaceDirection::NegativeY, block_type);
                        faces_rendered += 1;
                    }
                    if Self::should_render_face(chunk, world, x, y, z, FaceDirection::PositiveZ) {
                        mesh.add_face(x, y, z, FaceDirection::PositiveZ, block_type);
                        faces_rendered += 1;
                    }
                    if Self::should_render_face(chunk, world, x, y, z, FaceDirection::NegativeZ) {
                        mesh.add_face(x, y, z, FaceDirection::NegativeZ, block_type);
                        faces_rendered += 1;
                    }
                }
            }
        }
        
        println!("DEBUG: Chunk ({}, {}) - Solid blocks: {}, Faces rendered: {}", 
                 chunk.x, chunk.z, solid_blocks, faces_rendered);

        mesh
    }

    /// Check if a face should be rendered (face culling)
    fn should_render_face(chunk: &crate::world::chunk::Chunk, world: &crate::world::world_manager::World, x: i32, y: i32, z: i32, direction: FaceDirection) -> bool {
        let (check_x, check_y, check_z) = match direction {
            FaceDirection::PositiveX => (x + 1, y, z),
            FaceDirection::NegativeX => (x - 1, y, z),
            FaceDirection::PositiveY => (x, y + 1, z),
            FaceDirection::NegativeY => (x, y - 1, z),
            FaceDirection::PositiveZ => (x, y, z + 1),
            FaceDirection::NegativeZ => (x, y, z - 1),
        };

        // Check if neighbor is outside chunk bounds
        if !(0..16).contains(&check_x) || !(0..128).contains(&check_y) || !(0..16).contains(&check_z) {
            // Y coordinate is out of bounds, always render (world top/bottom)
            if !(0..128).contains(&check_y) {
                return true;
            }
            
            // Calculate neighboring chunk position
            let chunk_pos = crate::math::position::ChunkPos::new(chunk.x, chunk.z);
            let (neighbor_chunk_x, neighbor_chunk_z) = if check_x < 0 {
                (chunk_pos.x - 1, chunk_pos.z)
            } else if check_x >= 16 {
                (chunk_pos.x + 1, chunk_pos.z)
            } else if check_z < 0 {
                (chunk_pos.x, chunk_pos.z - 1)
            } else if check_z >= 16 {
                (chunk_pos.x, chunk_pos.z + 1)
            } else {
                (chunk_pos.x, chunk_pos.z) // Same chunk (shouldn't happen here)
            };

            let neighbor_chunk_pos = crate::math::position::ChunkPos::new(neighbor_chunk_x, neighbor_chunk_z);
            
            // Calculate local position in neighboring chunk
            let neighbor_local_x = if check_x < 0 { 15 } else if check_x >= 16 { 0 } else { check_x };
            let neighbor_local_z = if check_z < 0 { 15 } else if check_z >= 16 { 0 } else { check_z };
            let neighbor_local_pos = crate::math::position::LocalPos::new(neighbor_local_x, check_y, neighbor_local_z);

            // Check if neighboring chunk exists and get block
            if let Some(neighbor_chunk) = world.get_chunk(neighbor_chunk_pos) {
                neighbor_chunk.get_block(neighbor_local_pos) == 0 // Render if neighbor is air
            } else {
                true // No neighboring chunk, render the face
            }
        } else {
            // Check if neighbor block is air within the same chunk
            let neighbor_pos = crate::math::position::LocalPos::new(check_x, check_y, check_z);
            chunk.get_block(neighbor_pos) == 0
        }
    }

    /// Get color for block type
    fn get_block_color(&self, block_type: u8) -> [f32; 3] {
        match block_type {
            1 => [0.55, 0.55, 0.55], // Stone (gray)
            2 => [0.49, 0.70, 0.26], // Grass (green)
            3 => [0.55, 0.43, 0.39], // Dirt (brown)
            _ => [1.0, 0.0, 1.0], // Default (magenta)
        }
    }

    /// Get vertices reference
    pub fn get_vertices(&self) -> &Vec<BlockVertex> {
        &self.vertices
    }
    
    /// Get indices reference  
    pub fn get_indices(&self) -> &Vec<u32> {
        &self.indices
    }
    
    /// Get vertices (mutable)
    pub fn get_vertices_mut(&mut self) -> &mut Vec<BlockVertex> {
        &mut self.vertices
    }
    
    /// Get indices (mutable)
    pub fn get_indices_mut(&mut self) -> &mut Vec<u32> {
        &mut self.indices
    }

    /// Create vertex and index buffers for the mesh
    pub fn create_buffers(&self, device: &Device) -> (Buffer, Buffer, u32) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        (vertex_buffer, index_buffer, self.indices.len() as u32)
    }

    /// Get vertex buffer layout descriptor
    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        BlockVertex::desc()
    }
}

impl Default for ChunkMesh {
    fn default() -> Self {
        Self::new()
    }
}
