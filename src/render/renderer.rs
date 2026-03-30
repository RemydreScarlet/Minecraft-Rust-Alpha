//! Main renderer implementation
//! 
//! This module implements the main renderer equivalent to `e.java`.

use anyhow::Result;
use std::clone::Clone;
use winit::window::Window;
use wgpu::{util::DeviceExt, SurfaceConfiguration, Device, Queue, RenderPipeline, BindGroup, Buffer};
use glam::Mat4;
use crate::render::chunk_mesh::{ChunkMesh, BlockVertex};
use crate::world::world_manager::World;
use crate::camera::Camera;

/// Uniform buffer structure for matrices
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    proj_matrix: [[f32; 4]; 4],
    view_matrix: [[f32; 4]; 4],
    model_matrix: [[f32; 4]; 4],
}

/// Main renderer
pub struct Renderer {
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    uniform_buffer: Buffer,
    uniform_bind_group: BindGroup,
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
}

impl Renderer {
    /// Create a new renderer
    pub async fn new(window: &Window) -> Result<Self> {
        let size = window.inner_size();
        
        // WebGPU instance and adapter
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });
        
        let surface = unsafe { std::mem::transmute::<wgpu::Surface<'_>, wgpu::Surface<'_>>(instance.create_surface(window)?) };
        
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.ok_or_else(|| anyhow::anyhow!("Failed to find suitable adapter"))?;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        surface.configure(&device, &config);

        // Create depth texture
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create shaders
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Create uniform buffer for projection, view, and model matrices
        // Initial matrices will be set by camera on first frame
        let initial_camera = Camera::new(glam::Vec3::new(8.0, 40.0, 8.0));
        let projection_matrix = initial_camera.projection_matrix(size.width as f32 / size.height as f32);
        let view_matrix = initial_camera.view_matrix();
        let model_matrix = Mat4::IDENTITY;
        
        let uniforms = Uniforms {
            proj_matrix: projection_matrix.to_cols_array_2d(),
            view_matrix: view_matrix.to_cols_array_2d(),
            model_matrix: model_matrix.to_cols_array_2d(),
        };
        
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("uniform_bind_group_layout"),
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
            label: Some("uniform_bind_group"),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[BlockVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Create initial chunk mesh buffers (will be updated when world is available)
        let (vertex_buffer, index_buffer, num_indices) = create_empty_chunk_buffers(&device);

        Ok(Self {
            device,
            queue,
            config,
            size,
            surface,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            uniform_buffer,
            uniform_bind_group,
            depth_texture,
            depth_view,
        })
    }
    
    /// Update renderer with world data
    pub fn update_world(&mut self, world: &World) -> Result<()> {
        // Generate mesh for the first chunk (for now)
        if let Some(chunk) = world.get_all_chunks().first() {
            let chunk_mesh = ChunkMesh::generate_chunk_mesh(chunk);
            let (vertex_buffer, index_buffer, num_indices) = chunk_mesh.create_buffers(&self.device);
            
            self.vertex_buffer = vertex_buffer;
            self.index_buffer = index_buffer;
            self.num_indices = num_indices;
        }
        
        Ok(())
    }
    
    /// Update renderer with camera view matrix
    pub fn update_camera(&mut self, camera: &Camera) {
        let projection_matrix = camera.projection_matrix(self.size.width as f32 / self.size.height as f32);
        let view_matrix = camera.view_matrix();
        let model_matrix = Mat4::IDENTITY; // No world transformation
        
        let uniforms = Uniforms {
            proj_matrix: projection_matrix.to_cols_array_2d(),
            view_matrix: view_matrix.to_cols_array_2d(),
            model_matrix: model_matrix.to_cols_array_2d(),
        };
        
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }
    
    /// Render a frame
    pub fn render_frame(&mut self) -> Result<()> {
        let output = self.surface.get_current_texture()
            .map_err(|e| anyhow::anyhow!("Failed to get current texture: {:?}", e))?;
        
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Camera matrices are already updated by update_camera() call
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }

    /// Resize the renderer
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            // Recreate depth texture
            self.depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width: new_size.width,
                    height: new_size.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth24Plus,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            self.depth_view = self.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

            // Note: Camera matrices will be updated on next frame by update_camera()
            
            // Reconfigure surface
            self.surface.configure(&self.device, &self.config);
        }
    }
}

/// Create empty chunk buffers as placeholder
fn create_empty_chunk_buffers(device: &Device) -> (Buffer, Buffer, u32) {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Empty Vertex Buffer"),
        contents: bytemuck::cast_slice(&[BlockVertex { position: [0.0; 3], color: [0.0; 3] }]),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Empty Index Buffer"),
        contents: bytemuck::cast_slice(&[0u32]),
        usage: wgpu::BufferUsages::INDEX,
    });

    (vertex_buffer, index_buffer, 0)
}
