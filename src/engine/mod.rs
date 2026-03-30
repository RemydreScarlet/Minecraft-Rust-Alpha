//! Game engine module with WebGPU rendering support

use anyhow::Result;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent};
use pollster;

pub mod threading;

use crate::world::world::World;
use crate::engine::threading::ThreadManager;
use crate::render::Renderer;

/// Main game engine
pub struct Engine {
    thread_manager: Option<ThreadManager>,
}

impl Engine {
    /// Create a new engine instance
    pub fn new() -> Result<Self> {
        let thread_manager = Some(ThreadManager::new(crate::world::world::World::new(0)));
        
        Ok(Self {
            thread_manager,
        })
    }
    
    /// Run the main game loop with WebGPU rendering
    pub fn run(&mut self, world: World) -> Result<()> {
        let event_loop = EventLoop::new()?;
        
        let window = WindowBuilder::new()
            .with_title("Minecraft Alpha 1.1.2_01 - Rust")
            .with_inner_size(winit::dpi::LogicalSize::new(854, 480))
            .build(&event_loop)?;
        
        // Initialize renderer
        let mut renderer = pollster::block_on(Renderer::new(&window))?;
        
        // Update renderer with world data
        renderer.update_world(&world)?;
        
        println!("WebGPU initialized successfully");
        println!("Starting chunk-based world rendering...");
        
        let _thread_manager = self.thread_manager.take().unwrap();
        
        let _ = event_loop.run(move |event, window_target| {
            match event {
                Event::AboutToWait => {
                    window_target.set_control_flow(ControlFlow::Poll);
                    window.request_redraw();
                }
                
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("Window close requested");
                    std::process::exit(0);
                }
                
                Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    ..
                } => {
                    renderer.resize(new_size);
                }
                
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    if let Err(e) = renderer.render_frame() {
                        eprintln!("Render error: {:?}", e);
                    }
                }
                
                _ => (),
            }
        });
        
        Ok(())
    }
}
