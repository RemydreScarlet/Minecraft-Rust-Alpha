//! Main game loop implementation
//! 
//! This module implements the main game loop with 20 TPS timing,
//! equivalent to the main loop in Minecraft.java.

use std::time::{Duration, Instant};

/// Game timer that manages 20 TPS (ticks per second) timing
pub struct GameTimer {
    last_tick: Instant,
    tick_count: u64,
    partial_ticks: f32,
}

impl GameTimer {
    pub fn new() -> Self {
        Self {
            last_tick: Instant::now(),
            tick_count: 0,
            partial_ticks: 0.0,
        }
    }
    
    /// Update timer and return number of ticks to process
    pub fn update(&mut self) -> u32 {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        
        // Minecraft runs at 20 TPS, so each tick is 50ms
        let expected_ticks = elapsed.as_millis() as f32 / 50.0;
        let ticks_to_process = expected_ticks.floor() as u32;
        
        if ticks_to_process > 0 {
            self.last_tick = now;
            self.tick_count += ticks_to_process as u64;
            self.partial_ticks = expected_ticks - ticks_to_process as f32;
        } else {
            self.partial_ticks = expected_ticks;
        }
        
        ticks_to_process
    }
    
    /// Get partial tick value for smooth interpolation
    pub fn partial_ticks(&self) -> f32 {
        self.partial_ticks
    }
    
    /// Get total tick count
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }
}
