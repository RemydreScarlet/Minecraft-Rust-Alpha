//! Game engine core module
//! 
//! This module contains the main game loop, input management, and display management.

use anyhow::Result;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{Window, WindowBuilder};
use winit::event::{Event, WindowEvent};
use wgpu;

use crate::world::world::World;
use crate::render::Renderer;

/// Main game engine
pub struct Engine {
    window: Option<Window>,
}

impl Engine {
    /// Create a new engine instance
    pub fn new() -> Result<Self> {
        Ok(Self {
            window: None,
        })
    }
    
    /// Run the main game loop with actual window
    pub fn run(&mut self, mut world: World, _renderer: Renderer) -> Result<()> {
        println!("Creating window: Minecraft Alpha 1.1.2_01 - Rust");
        println!("Window size: 854x480");
        
        let event_loop = EventLoop::new()?;
        
        let window = WindowBuilder::new()
            .with_title("Minecraft Alpha 1.1.2_01 - Rust")
            .with_inner_size(winit::dpi::LogicalSize::new(854, 480))
            .build(&event_loop)?;
        
        // Initialize wgpu context
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });
        
        let surface = instance.create_surface(&window)?;
        
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })).ok_or_else(|| anyhow::anyhow!("Failed to find suitable adapter"))?;
        
        // capabilitiesを先に取得
        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities.formats[0];
        
        let (device, _queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )).map_err(|e| anyhow::anyhow!("Failed to create device: {:?}", e))?;
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: 854,
            height: 480,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        surface.configure(&device, &config);
        
        // surfaceをドロップしてwindowの借用を解放
        drop(surface);
        
        // ここでwindowを設定
        self.window = Some(window);
        
        println!("WGPU context initialized successfully");
        
        // Game timing variables
        let target_tps = 20; // 20 TPS
        let tick_duration = std::time::Duration::from_millis(1000 / target_tps);
        let mut last_tick = std::time::Instant::now();
        let mut should_exit = false;
        
        println!("Starting game loop at {} TPS", target_tps);
        
        // Run the event loop
        let _ = event_loop.run(move |event, window_target| {
            match event {
                Event::AboutToWait => {
                    if should_exit {
                        // イベントループを終了させる
                        std::process::exit(0);
                    }
                    
                    let now = std::time::Instant::now();
                    
                    // Check if it's time for a game tick
                    if now.duration_since(last_tick) >= tick_duration {
                        // Update game logic at fixed 20 TPS
                        world.update();
                        last_tick = now;
                        
                        println!("Game tick at 20 TPS");
                    }
                    
                    window_target.set_control_flow(ControlFlow::Poll);
                }
                
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("Window close requested, exiting game loop");
                    should_exit = true;
                }
                
                _ => (),
            }
        });
        
        println!("Game loop completed successfully!");
        Ok(())
    }
}
