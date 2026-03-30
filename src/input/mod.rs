//! Input handling system
//! 
//! Tracks keyboard and mouse state for camera control

use winit::event::{MouseButton, MouseScrollDelta, WindowEvent};

/// Input state tracking
#[derive(Debug, Default)]
pub struct InputState {
    // Movement keys
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
    
    // Mouse state
    pub mouse_captured: bool,
    pub mouse_delta_x: f32,
    pub mouse_delta_y: f32,
    pub last_mouse_x: f64,
    pub last_mouse_y: f64,
    
    // Window focus
    pub window_focused: bool,
    
    // Capture state change flag
    pub capture_changed: bool,
}

impl InputState {
    /// Create a new input state
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Process a window event and update input state
    pub fn process_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::Focused(focused) => {
                self.window_focused = *focused;
                if !*focused {
                    // Release mouse capture when window loses focus
                    if self.mouse_captured {
                        self.mouse_captured = false;
                        self.capture_changed = true;
                    }
                }
                false
            }
            
            WindowEvent::KeyboardInput { event, .. } => {
                self.process_keyboard_input(event)
            }
            
            WindowEvent::MouseInput { state, button, .. } => {
                self.process_mouse_input(*state, *button)
            }
            
            WindowEvent::CursorMoved { position, .. } => {
                self.process_cursor_moved(position.x, position.y)
            }
            
            WindowEvent::MouseWheel { delta, .. } => {
                self.process_mouse_scroll(delta);
                false
            }
            
            _ => false,
        }
    }
    
    /// Process keyboard input
    fn process_keyboard_input(&mut self, input: &winit::event::KeyEvent) -> bool {
        let pressed = input.state == winit::event::ElementState::Pressed;
        
        match &input.logical_key {
            winit::keyboard::Key::Named(key) => {
                match key {
                    winit::keyboard::NamedKey::Escape => {
                        if pressed {
                            if self.mouse_captured {
                                self.mouse_captured = false;
                                self.capture_changed = true;
                            }
                        }
                        true
                    }
                    winit::keyboard::NamedKey::Space => {
                        self.move_up = pressed;
                        true
                    }
                    winit::keyboard::NamedKey::Shift => {
                        self.move_down = pressed;
                        true
                    }
                    _ => false,
                }
            }
            winit::keyboard::Key::Character(ch) => {
                match ch.as_str() {
                    "w" | "W" => {
                        self.move_forward = pressed;
                        true
                    }
                    "s" | "S" => {
                        self.move_backward = pressed;
                        true
                    }
                    "a" | "A" => {
                        self.move_left = pressed;
                        true
                    }
                    "d" | "D" => {
                        self.move_right = pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    
    /// Process mouse button input
    fn process_mouse_input(&mut self, state: winit::event::ElementState, button: MouseButton) -> bool {
        if button == MouseButton::Left && state == winit::event::ElementState::Pressed {
            if !self.mouse_captured {
                self.mouse_captured = true;
                self.capture_changed = true;
            }
            true
        } else {
            false
        }
    }
    
    /// Process cursor movement
    fn process_cursor_moved(&mut self, x: f64, y: f64) -> bool {
        if self.mouse_captured {
            // Calculate mouse delta
            self.mouse_delta_x = (x - self.last_mouse_x) as f32;
            self.mouse_delta_y = (y - self.last_mouse_y) as f32;
            
            // Update last position
            self.last_mouse_x = x;
            self.last_mouse_y = y;
            
            true
        } else {
            // Update last position for next capture
            self.last_mouse_x = x;
            self.last_mouse_y = y;
            
            // Reset delta when not captured
            self.mouse_delta_x = 0.0;
            self.mouse_delta_y = 0.0;
            
            false
        }
    }
    
    /// Process mouse scroll (currently unused but available)
    fn process_mouse_scroll(&mut self, _delta: &MouseScrollDelta) -> bool {
        // Could be used for zoom or other features
        false
    }
    
    /// Reset mouse delta after processing
    pub fn reset_mouse_delta(&mut self) {
        self.mouse_delta_x = 0.0;
        self.mouse_delta_y = 0.0;
        self.capture_changed = false;
    }
    
    /// Check if any movement key is pressed
    pub fn is_moving(&self) -> bool {
        self.move_forward || self.move_backward || 
        self.move_left || self.move_right || 
        self.move_up || self.move_down
    }
}
