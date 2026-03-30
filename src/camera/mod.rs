//! Camera system implementation
//! 
//! Provides free-look camera with WASD movement and mouse control

use glam::{Mat4, Vec3, Vec2};
use crate::math::utils::{clamp, deg_to_rad};

/// Camera with free-look movement
pub struct Camera {
    /// Camera position in world space
    pub position: Vec3,
    /// Camera rotation in degrees (pitch, yaw)
    pub rotation: Vec2,
    /// Movement speed in units per second
    pub movement_speed: f32,
    /// Mouse sensitivity in degrees per pixel
    pub mouse_sensitivity: f32,
    /// Field of view in degrees
    pub fov: f32,
}

impl Camera {
    /// Create a new camera at the given position
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation: Vec2::new(0.0, 0.0), // pitch, yaw
            movement_speed: 5.0,
            mouse_sensitivity: 0.1,
            fov: 45.0,
        }
    }
    
    /// Get the forward vector based on current rotation
    pub fn forward(&self) -> Vec3 {
        let pitch_rad = deg_to_rad(self.rotation.x);
        let yaw_rad = deg_to_rad(self.rotation.y);
        
        Vec3::new(
            yaw_rad.sin() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.cos() * pitch_rad.cos(),
        ).normalize()
    }
    
    /// Get the right vector based on current rotation
    pub fn right(&self) -> Vec3 {
        let forward = self.forward();
        let up = Vec3::new(0.0, 1.0, 0.0);
        forward.cross(up).normalize()
    }
    
    /// Get the up vector (always world up for free camera)
    pub fn up(&self) -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }
    
    /// Process keyboard input for movement
    pub fn process_keyboard(&mut self, input: &crate::input::InputState, delta_time: f32) {
        let forward = self.forward();
        let right = self.right();
        let up = self.up();
        
        let mut movement = Vec3::ZERO;
        
        if input.move_forward {
            movement += forward;
        }
        if input.move_backward {
            movement -= forward;
        }
        if input.move_left {
            movement -= right;
        }
        if input.move_right {
            movement += right;
        }
        if input.move_up {
            movement += up;
        }
        if input.move_down {
            movement -= up;
        }
        
        // Normalize diagonal movement
        if movement.length_squared() > 0.0 {
            movement = movement.normalize();
        }
        
        // Apply movement with speed and delta time
        self.position += movement * self.movement_speed * delta_time;
    }
    
    /// Process mouse movement for looking
    pub fn process_mouse(&mut self, delta_x: f32, delta_y: f32) {
        // Update yaw and pitch based on mouse movement
        self.rotation.y += delta_x * self.mouse_sensitivity;
        self.rotation.x += delta_y * self.mouse_sensitivity;
        
        // Clamp pitch to prevent camera flip
        self.rotation.x = clamp(self.rotation.x, -89.0, 89.0);
        
        // Normalize yaw to 0-360 range
        self.rotation.y %= 360.0;
    }
    
    /// Calculate the view matrix for this camera
    pub fn view_matrix(&self) -> Mat4 {
        let forward = self.forward();
        let up = self.up();
        
        // Create look-at matrix
        Mat4::look_at_rh(
            self.position,
            self.position + forward,
            up,
        )
    }
    
    /// Calculate projection matrix with given aspect ratio
    pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::perspective_rh_gl(
            self.fov.to_radians(),
            aspect_ratio,
            0.1,
            1000.0,
        )
    }
}
