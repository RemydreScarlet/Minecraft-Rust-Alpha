//! HUD (Heads-Up Display) system for debug information
//! 
//! Provides F3 debug screen with FPS, coordinates, and system information

use std::time::{Instant, Duration};
use wgpu::{Device, SurfaceConfiguration};
use crate::entities::player::Player;
use crate::world::world_manager::World;
use crate::camera::Camera;

/// Debug information display state
#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub visible: bool,
    pub fps: f32,
    pub frame_time: f32,
    pub player_pos: String,
    pub chunk_pos: String,
    pub looking_at: String,
    pub webgpu_info: WebGPUInfo,
}

/// WebGPU adapter and device information
#[derive(Debug, Clone)]
pub struct WebGPUInfo {
    pub adapter_name: String,
    pub backend: String,
    pub device_type: String,
    pub limits: String,
}

impl Default for DebugInfo {
    fn default() -> Self {
        Self {
            visible: false,
            fps: 0.0,
            frame_time: 0.0,
            player_pos: "Unknown".to_string(),
            chunk_pos: "Unknown".to_string(),
            looking_at: "Air".to_string(),
            webgpu_info: WebGPUInfo {
                adapter_name: "Unknown".to_string(),
                backend: "Unknown".to_string(),
                device_type: "Unknown".to_string(),
                limits: "Unknown".to_string(),
            },
        }
    }
}

/// HUD system for rendering debug information
pub struct HUD {
    debug_info: DebugInfo,
    last_frame_time: Instant,
    frame_times: Vec<Duration>,
    max_frame_samples: usize,
}

impl HUD {
    /// Create a new HUD system
    pub fn new() -> Self {
        Self {
            debug_info: DebugInfo::default(),
            last_frame_time: Instant::now(),
            frame_times: Vec::new(),
            max_frame_samples: 60, // Average over 60 frames
        }
    }
    
    /// Toggle debug display visibility
    pub fn toggle_debug(&mut self) {
        self.debug_info.visible = !self.debug_info.visible;
    }
    
    /// Update debug information for current frame
    pub fn update(&mut self, player: &Player, _world: &World, _camera: &Camera) {
        // Calculate FPS and frame time
        let now = Instant::now();
        let frame_duration = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;
        
        // Store frame times for averaging
        self.frame_times.push(frame_duration);
        if self.frame_times.len() > self.max_frame_samples {
            self.frame_times.remove(0);
        }
        
        // Calculate average frame time and FPS
        let avg_frame_time: Duration = self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32;
        self.debug_info.frame_time = avg_frame_time.as_secs_f32() * 1000.0; // Convert to milliseconds
        self.debug_info.fps = 1.0 / avg_frame_time.as_secs_f32();
        
        // Update player position information
        let (px, py, pz) = player.get_position();
        let world_pos = player.get_world_pos();
        let chunk_pos = player.get_chunk_pos();
        let local_pos = player.get_local_pos();
        
        self.debug_info.player_pos = format!(
            "XYZ: {:.2} / {:.2} / {:.2} ({} {} {})",
            px, py, pz,
            world_pos.x, world_pos.y, world_pos.z
        );
        
        self.debug_info.chunk_pos = format!(
            "Chunk: {} {} (local: {} {} {})",
            chunk_pos.x, chunk_pos.z,
            local_pos.x, local_pos.y, local_pos.z
        );
        
        // Update looking at information (simplified for now)
        self.debug_info.looking_at = "Looking at: Air".to_string();
    }
    
    /// Update WebGPU information
    pub fn update_webgpu_info(&mut self, _device: &Device, config: &SurfaceConfiguration) {
        self.debug_info.webgpu_info = WebGPUInfo {
            adapter_name: "WebGPU Device".to_string(), // Simplified for now
            backend: "WebGPU".to_string(),
            device_type: "GPU".to_string(),
            limits: format!("Surface: {}x{}", config.width, config.height),
        };
    }
    
    /// Get current debug information
    pub fn get_debug_info(&self) -> &DebugInfo {
        &self.debug_info
    }
    
    /// Generate debug text for console output
    pub fn generate_debug_text(&self) -> String {
        if !self.debug_info.visible {
            return String::new();
        }
        
        format!(
            "=== DEBUG INFO ===\n\
             FPS: {:.1} (Frame time: {:.2}ms)\n\
             {}\n\
             {}\n\
             {}\n\
             WebGPU: {}\n\
             Backend: {}\n\
             Device: {}\n\
             {}\n\
             =================",
            self.debug_info.fps,
            self.debug_info.frame_time,
            self.debug_info.player_pos,
            self.debug_info.chunk_pos,
            self.debug_info.looking_at,
            self.debug_info.webgpu_info.adapter_name,
            self.debug_info.webgpu_info.backend,
            self.debug_info.webgpu_info.device_type,
            self.debug_info.webgpu_info.limits
        )
    }
}

impl Default for HUD {
    fn default() -> Self {
        Self::new()
    }
}
