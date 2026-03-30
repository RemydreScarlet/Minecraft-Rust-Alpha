//! Main renderer implementation
//! 
//! This module implements the main renderer equivalent to `e.java`.

use anyhow::Result;
use std::clone::Clone;
use winit::window::Window;
use wgpu::{util::DeviceExt, SurfaceConfiguration, Device, Queue, RenderPipeline, BindGroup, Buffer};
use glam::Mat4;

/// Vertex structure for 3D rendering
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

/// Uniform buffer structure for matrices
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    proj_matrix: [[f32; 4]; 4],
    view_matrix: [[f32; 4]; 4],
    model_matrix: [[f32; 4]; 4],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
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
    start_time: std::time::Instant,
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
        
        let surface = unsafe { std::mem::transmute(instance.create_surface(window)?) };
        
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
        let projection_matrix = create_projection_matrix(size.width as f32 / size.height as f32);
        let view_matrix = create_view_matrix();
        let model_matrix = create_model_matrix(0.0); // Initial static model
        
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
                buffers: &[Vertex::desc()],
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

        // Create basic cube vertices
        let (vertex_buffer, index_buffer, num_indices) = create_cube_buffers(&device);

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
            start_time: std::time::Instant::now(),
        })
    }
    
    /// Render a frame
    pub fn render_frame(&mut self) -> Result<()> {
        let output = self.surface.get_current_texture()
            .map_err(|e| anyhow::anyhow!("Failed to get current texture: {:?}", e))?;
        
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Update model matrix with rotation animation
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let model_matrix = create_model_matrix(elapsed);
        
        // Update uniform buffer with animated model matrix
        let projection_matrix = create_projection_matrix(self.size.width as f32 / self.size.height as f32);
        let view_matrix = create_view_matrix();
        
        let uniforms = Uniforms {
            proj_matrix: projection_matrix.to_cols_array_2d(),
            view_matrix: view_matrix.to_cols_array_2d(),
            model_matrix: model_matrix.to_cols_array_2d(),
        };
        
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        
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

            // Update projection matrix
            let projection_matrix = create_projection_matrix(new_size.width as f32 / new_size.height as f32);
            let view_matrix = create_view_matrix();
            let model_matrix = create_model_matrix(0.0); // Reset to static for resize
            
            let uniforms = Uniforms {
                proj_matrix: projection_matrix.to_cols_array_2d(),
                view_matrix: view_matrix.to_cols_array_2d(),
                model_matrix: model_matrix.to_cols_array_2d(),
            };
            
            self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
            
            // Reconfigure surface
            self.surface.configure(&self.device, &self.config);
        }
    }
}

/// Create projection matrix
fn create_projection_matrix(aspect_ratio: f32) -> Mat4 {
    Mat4::perspective_rh_gl(45.0f32.to_radians(), aspect_ratio, 0.1, 100.0)
}

/// Create view matrix (camera)
fn create_view_matrix() -> Mat4 {
    Mat4::look_at_rh(
        glam::Vec3::new(3.0, 2.0, 5.0),  // Camera position
        glam::Vec3::new(0.0, 0.0, 0.0),   // Look at origin
        glam::Vec3::new(0.0, 1.0, 0.0),   // Up vector
    )
}

/// Create model matrix (object transformation)
fn create_model_matrix(time: f32) -> Mat4 {
    let rotation_y = Mat4::from_rotation_y(time);
    let rotation_x = Mat4::from_rotation_x(time * 0.7);
    let translation = Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 0.0));
    translation * rotation_y * rotation_x
}

/// Create basic cube vertex and index buffers
fn create_cube_buffers(device: &Device) -> (Buffer, Buffer, u32) {
    let vertices = vec![
        // Front face (red) - Z+面
        Vertex { position: [-1.0, -1.0,  1.0], color: [1.0, 0.0, 0.0] }, // 0
        Vertex { position: [ 1.0, -1.0,  1.0], color: [1.0, 0.0, 0.0] }, // 1
        Vertex { position: [ 1.0,  1.0,  1.0], color: [1.0, 0.0, 0.0] }, // 2
        Vertex { position: [-1.0,  1.0,  1.0], color: [1.0, 0.0, 0.0] }, // 3
        
        // Back face (green) - Z-面
        Vertex { position: [ 1.0, -1.0, -1.0], color: [0.0, 1.0, 0.0] }, // 4
        Vertex { position: [-1.0, -1.0, -1.0], color: [0.0, 1.0, 0.0] }, // 5
        Vertex { position: [-1.0,  1.0, -1.0], color: [0.0, 1.0, 0.0] }, // 6
        Vertex { position: [ 1.0,  1.0, -1.0], color: [0.0, 1.0, 0.0] }, // 7
        
        // Top face (blue) - Y+面
        Vertex { position: [-1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] }, // 8
        Vertex { position: [ 1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] }, // 9
        Vertex { position: [ 1.0,  1.0, -1.0], color: [0.0, 0.0, 1.0] }, // 10
        Vertex { position: [-1.0,  1.0, -1.0], color: [0.0, 0.0, 1.0] }, // 11
        
        // Bottom face (yellow) - Y-面
        Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 1.0, 0.0] }, // 12
        Vertex { position: [ 1.0, -1.0, -1.0], color: [1.0, 1.0, 0.0] }, // 13
        Vertex { position: [ 1.0, -1.0,  1.0], color: [1.0, 1.0, 0.0] }, // 14
        Vertex { position: [-1.0, -1.0,  1.0], color: [1.0, 1.0, 0.0] }, // 15
        
        // Right face (magenta) - X+面
        Vertex { position: [ 1.0, -1.0,  1.0], color: [1.0, 0.0, 1.0] }, // 16
        Vertex { position: [ 1.0, -1.0, -1.0], color: [1.0, 0.0, 1.0] }, // 17
        Vertex { position: [ 1.0,  1.0, -1.0], color: [1.0, 0.0, 1.0] }, // 18
        Vertex { position: [ 1.0,  1.0,  1.0], color: [1.0, 0.0, 1.0] }, // 19
        
        // Left face (cyan) - X-面
        Vertex { position: [-1.0, -1.0, -1.0], color: [0.0, 1.0, 1.0] }, // 20
        Vertex { position: [-1.0, -1.0,  1.0], color: [0.0, 1.0, 1.0] }, // 21
        Vertex { position: [-1.0,  1.0,  1.0], color: [0.0, 1.0, 1.0] }, // 22
        Vertex { position: [-1.0,  1.0, -1.0], color: [0.0, 1.0, 1.0] }, // 23
    ];

    let indices = vec![
        // Front face (Z+) - 時計回りに修正
        0, 1, 2, 2, 3, 0,
        // Back face (Z-) - 時計回りに修正
        4, 5, 6, 6, 7, 4,
        // Top face (Y+) - 時計回りに修正
        8, 9, 10, 10, 11, 8,
        // Bottom face (Y-) - 時計回りに修正
        12, 13, 14, 14, 15, 12,
        // Right face (X+) - 時計回りに修正
        16, 17, 18, 18, 19, 16,
        // Left face (X-) - 時計回りに修正
        20, 21, 22, 22, 23, 20,
    ];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    (vertex_buffer, index_buffer, indices.len() as u32)
}
