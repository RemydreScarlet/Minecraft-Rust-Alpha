//! Frustum culling implementation
//! 
//! This module implements frustum culling for optimization.

use crate::math::position::WorldPos;

/// Frustum for view frustum culling
pub struct Frustum {
    /// Frustum planes
    planes: [f32; 6],
}

impl Frustum {
    /// Create a new frustum
    pub fn new() -> Self {
        Self {
            planes: [0.0; 6],
        }
    }
    
    /// Check if position is in frustum
    pub fn contains(&self, pos: WorldPos) -> bool {
        // Simple frustum check - always return true for now
        true
    }
}
