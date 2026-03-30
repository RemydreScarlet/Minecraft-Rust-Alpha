//! Game engine core module
//! 
//! This module contains the main game loop, input management, and display management.
//! Now with multithreading support for improved performance.

use anyhow::Result;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{Window, WindowBuilder};
use winit::event::{Event, WindowEvent};
use wgpu;
use std::sync::mpsc::TryRecvError;

pub mod threading;

use crate::world::world::World;
use crate::render::Renderer;
use crate::engine::threading::{ThreadManager, GameMessage, WorldResponse};

/// Main game engine with multithreading support
pub struct Engine {
    window: Option<Window>,
    thread_manager: Option<ThreadManager>,
    renderer: Option<Renderer>,
    partial_ticks: f32,
    last_world_tick: u64,
}

impl Engine {
    /// Create a new engine instance
    pub fn new() -> Result<Self> {
        Ok(Self {
            window: None,
            thread_manager: None,
            renderer: None,
            partial_ticks: 0.0,
            last_world_tick: 0,
        })
    }
    
    /// Run main game loop with multithreading support
    pub fn run(&mut self, world: World) -> Result<()> {
        println!("Creating window: Minecraft Alpha 1.1.2_01 - Rust (Multithreaded)");
        println!("Window size: 854x480");
        
        // Initialize thread manager and start world thread
        let mut thread_manager = ThreadManager::new(world);
        thread_manager.start_world_thread()
            .map_err(|e| anyhow::anyhow!("Failed to start world thread: {:?}", e))?;
        
        println!("World thread started successfully");
        
        self.thread_manager = Some(thread_manager);
        
        let event_loop = EventLoop::new()?;
        
        // Move ownership to closure
        let window = WindowBuilder::new()
            .with_title("Minecraft Alpha 1.1.2_01 - Rust")
            .with_inner_size(winit::dpi::LogicalSize::new(854, 480))
            .build(&event_loop)?;
        
        // Initialize wgpu context after window creation
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
        
        // Initialize renderer with window
        let renderer = pollster::block_on(Renderer::new(&window))?;
        
        println!("WGPU context and renderer initialized successfully");
        
        // Send initial world tick to start game loop
        if let Some(ref mut thread_manager) = self.thread_manager {
            thread_manager.send_world_message(GameMessage::WorldTick)
                .map_err(|e| anyhow::anyhow!("Failed to send initial world tick: {:?}", e))?;
        }
        
        println!("Starting multithreaded game loop");
        
        // Move ownership to closure
        let mut renderer = pollster::block_on(Renderer::new(&window))?;
        let mut thread_manager = self.thread_manager.take().unwrap();
        
        let _ = event_loop.run(move |event, window_target| {
            match event {
                Event::AboutToWait => {
                    // Process world thread responses
                    match thread_manager.try_recv_world_response() {
                        Ok(WorldResponse::TickCompleted { tick_count, partial_ticks }) => {
                            // Request next tick
                            let _ = thread_manager.send_world_message(GameMessage::WorldTick);
                            
                            if tick_count % 100 == 0 {
                                println!("World tick: {} (partial: {:.2})", tick_count, partial_ticks);
                            }
                        }
                        Ok(WorldResponse::WorldStateSnapshot { .. }) => {
                            // Handle world state updates for rendering
                        }
                        Ok(WorldResponse::BlockOperationResult { .. }) => {
                            // Handle block operation results
                        }
                        Err(TryRecvError::Empty) => {
                            // No messages from world thread, continue
                        }
                        Err(TryRecvError::Disconnected) => {
                            println!("World thread disconnected, shutting down");
                            std::process::exit(0);
                        }
                    }
                    
                    // Render frame
                    if let Err(e) = renderer.render_frame() {
                        println!("Render error: {:?}", e);
                    }
                    
                    window_target.set_control_flow(ControlFlow::Poll);
                }
                
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("Window close requested, shutting down threads");
                    let _ = thread_manager.send_world_message(GameMessage::Shutdown);
                    std::process::exit(0);
                }
                
                _ => (),
            }
        });
        
        println!("Game loop completed successfully!");
        Ok(())
    }
}
