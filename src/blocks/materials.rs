//! Block materials implementation
//! 
//! This module implements block materials equivalent to `ao.java`.

/// Block material
pub struct BlockMaterial {
    pub name: String,
    pub solid: bool,
    pub transparent: bool,
}

impl BlockMaterial {
    /// Create a new block material
    pub fn new(name: &str, solid: bool, transparent: bool) -> Self {
        Self {
            name: name.to_string(),
            solid,
            transparent,
        }
    }
}
