//! Game engine module with WebGPU rendering support

use anyhow::Result;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent};
use pollster;
use std::time::Instant;

pub mod threading;

use crate::world::world::World;
use crate::engine::threading::ThreadManager;
use crate::render::Renderer;
use crate::camera::Camera;
use crate::input::InputState;

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
        
        // Initialize camera and input
        let mut camera = Camera::new(glam::Vec3::new(8.0, 40.0, 8.0));
        let mut input = InputState::new();
        let mut last_time = Instant::now();
        
        println!("WebGPU initialized successfully");
        println!("Starting chunk-based world rendering...");
        println!("Controls: WASD to move, Space/Shift to fly up/down, Mouse to look, Click to capture mouse, Escape to release");
        
        let _thread_manager = self.thread_manager.take().unwrap();
        
        let _ = event_loop.run(move |event, window_target| {
            match event {
                Event::AboutToWait => {
                    // Calculate delta time
                    let current_time = Instant::now();
                    let delta_time = current_time.duration_since(last_time).as_secs_f32();
                    last_time = current_time;
                    
                    // Handle mouse capture state changes
                    if input.capture_changed {
                        if input.mouse_captured {
                            // Capture mouse - try different modes
                            let grab_result = window.set_cursor_grab(winit::window::CursorGrabMode::Locked);
                            if let Err(e) = grab_result {
                                eprintln!("Failed to lock cursor: {:?}", e);
                                // Try Confined mode as fallback
                                if let Err(e2) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                                    eprintln!("Failed to confine cursor: {:?}", e2);
                                    // Final fallback: just hide cursor
                                    window.set_cursor_visible(false);
                                } else {
                                    window.set_cursor_visible(false);
                                }
                            } else {
                                window.set_cursor_visible(false);
                            }
                        } else {
                            // Release mouse
                            if let Err(e) = window.set_cursor_grab(winit::window::CursorGrabMode::None) {
                                eprintln!("Failed to release cursor: {:?}", e);
                            }
                            window.set_cursor_visible(true);
                        }
                    }
                    
                    // Update camera based on input
                    camera.process_keyboard(&input, delta_time);
                    camera.process_mouse(input.mouse_delta_x, input.mouse_delta_y);
                    
                    // Update renderer with camera view matrix
                    renderer.update_camera(&camera);
                    
                    // Reset mouse delta after processing
                    input.reset_mouse_delta();
                    
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
                
                Event::WindowEvent {
                    event: ref window_event,
                    ..
                } => {
                    // Process input events
                    input.process_event(window_event);
                }
                
                _ => (),
            }
        });
        
        Ok(())
    }
}
