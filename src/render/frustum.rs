//! Frustum culling implementation
//! 
//! This module implements frustum culling for optimization.

use crate::math::position::WorldPos;

/// Frustum for view frustum culling
pub struct Frustum {
}

impl Frustum {
    /// Create a new frustum
    pub fn new() -> Self {
        Self {}
    }
    
    /// Check if position is in frustum
    pub fn contains(&self, _pos: WorldPos) -> bool {
        // Simple frustum check - always return true for now
        true
    }
}

impl Default for Frustum {
    fn default() -> Self {
        Self::new()
    }
}
