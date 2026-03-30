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
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    uniform_buffer: Buffer,
    uniform_bind_group: BindGroup,
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
        
        let surface = instance.create_surface(window)?;
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

        // Create shaders
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Create uniform buffer for projection matrix
        let projection_matrix = create_projection_matrix(size.width as f32 / size.height as f32);
        let matrix_array: [[f32; 4]; 4] = projection_matrix.to_cols_array_2d();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&matrix_array),
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
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            uniform_buffer,
            uniform_bind_group,
        })
    }
    
    /// Render a frame
    pub fn render_frame(&mut self) -> Result<()> {
        // For now, just return Ok since we don't have surface
        Ok(())
    }

    /// Resize the renderer
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            // Update projection matrix
            let projection_matrix = create_projection_matrix(new_size.width as f32 / new_size.height as f32);
            let matrix_array: [[f32; 4]; 4] = projection_matrix.to_cols_array_2d();
            self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&matrix_array));
        }
    }
}

/// Create projection matrix
fn create_projection_matrix(aspect_ratio: f32) -> Mat4 {
    Mat4::perspective_rh_gl(45.0f32.to_radians(), aspect_ratio, 0.1, 100.0)
}

/// Create basic cube vertex and index buffers
fn create_cube_buffers(device: &Device) -> (Buffer, Buffer, u32) {
    let vertices = vec![
        // Front face (red)
        Vertex { position: [-1.0, -1.0,  1.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [ 1.0, -1.0,  1.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [ 1.0,  1.0,  1.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [-1.0,  1.0,  1.0], color: [1.0, 0.0, 0.0] },
        // Back face (green)
        Vertex { position: [-1.0, -1.0, -1.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [ 1.0, -1.0, -1.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [ 1.0,  1.0, -1.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [-1.0,  1.0, -1.0], color: [0.0, 1.0, 0.0] },
        // Top face (blue)
        Vertex { position: [-1.0,  1.0, -1.0], color: [0.0, 0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0, -1.0], color: [0.0, 0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] },
        Vertex { position: [-1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] },
        // Bottom face (yellow)
        Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 1.0, 0.0] },
        Vertex { position: [ 1.0, -1.0, -1.0], color: [1.0, 1.0, 0.0] },
        Vertex { position: [ 1.0, -1.0,  1.0], color: [1.0, 1.0, 0.0] },
        Vertex { position: [-1.0, -1.0,  1.0], color: [1.0, 1.0, 0.0] },
        // Right face (magenta)
        Vertex { position: [ 1.0, -1.0, -1.0], color: [1.0, 0.0, 1.0] },
        Vertex { position: [ 1.0, -1.0,  1.0], color: [1.0, 0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0,  1.0], color: [1.0, 0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0, -1.0], color: [1.0, 0.0, 1.0] },
        // Left face (cyan)
        Vertex { position: [-1.0, -1.0, -1.0], color: [0.0, 1.0, 1.0] },
        Vertex { position: [-1.0, -1.0,  1.0], color: [0.0, 1.0, 1.0] },
        Vertex { position: [-1.0,  1.0,  1.0], color: [0.0, 1.0, 1.0] },
        Vertex { position: [-1.0,  1.0, -1.0], color: [0.0, 1.0, 1.0] },
    ];

    let indices = vec![
        0, 1, 2, 2, 3, 0,  // front
        4, 6, 5, 6, 4, 7,  // back
        8, 9, 10, 10, 11, 8,  // top
        12, 14, 13, 14, 12, 15,  // bottom
        16, 17, 18, 18, 19, 16,  // right
        20, 22, 21, 22, 20, 23,  // left
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
