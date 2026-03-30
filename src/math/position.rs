//! Position and coordinate system implementation
//! 
//! This module implements the Position struct with packed coordinates,
//! equivalent to the `a` class in the original Java code.

use std::fmt;

/// Immutable 3D integer coordinate for block positions
/// 
/// This is equivalent to the `a` class in the original Java code.
/// Uses bit manipulation for efficient storage and hashing.
/// 
/// # Packing Format
/// - X: 10 bits (0-1023)
/// - Y: 10 bits (0-1023) 
/// - Z: 10 bits (0-1023)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// Packed coordinate (X | Y<<10 | Z<<20)
    packed: i32,
}

impl Position {
    /// Create a new position from x, y, z coordinates
    /// 
    /// # Arguments
    /// * `x` - X coordinate
    /// * `y` - Y coordinate  
    /// * `z` - Z coordinate
    /// 
    /// # Panics
    /// Panics if any coordinate is outside the range 0-1023
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        assert!((0..1024).contains(&x), "X coordinate out of range: {}", x);
        assert!((0..1024).contains(&y), "Y coordinate out of range: {}", y);
        assert!((0..1024).contains(&z), "Z coordinate out of range: {}", z);
        
        let packed = x | (y << 10) | (z << 20);
        Self { packed }
    }
    
    /// Get the X coordinate
    pub fn x(&self) -> i32 {
        self.packed & 0x3FF  // 10 bits
    }
    
    /// Get the Y coordinate
    pub fn y(&self) -> i32 {
        (self.packed >> 10) & 0x3FF  // 10 bits
    }
    
    /// Get the Z coordinate
    pub fn z(&self) -> i32 {
        (self.packed >> 20) & 0x3FF  // 10 bits
    }
    
    /// Get the packed coordinate value
    pub fn packed(&self) -> i32 {
        self.packed
    }
    
    /// Calculate distance between two positions
    /// 
    /// This uses Manhattan distance as in the original game
    pub fn distance_to(&self, other: &Position) -> i32 {
        let dx = (self.x() - other.x()).abs();
        let dy = (self.y() - other.y()).abs();
        let dz = (self.z() - other.z()).abs();
        dx + dy + dz
    }
    
    /// Create a position from a packed coordinate
    pub fn from_packed(packed: i32) -> Self {
        Self { packed }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

/// World coordinate for block positions in the game world
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorldPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl WorldPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    
    /// Convert world coordinates to chunk coordinates
    pub fn to_chunk_pos(&self) -> ChunkPos {
        ChunkPos {
            x: self.x >> 4,  // Divide by 16
            z: self.z >> 4,  // Divide by 16
        }
    }
    
    /// Convert world coordinates to local coordinates within a chunk
    pub fn to_local_pos(&self) -> LocalPos {
        LocalPos {
            x: self.x & 15,  // Mod 16
            y: self.y,
            z: self.z & 15,  // Mod 16
        }
    }
}

impl fmt::Display for WorldPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Chunk coordinate (16x16 blocks)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

impl fmt::Display for ChunkPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.z)
    }
}

/// Local coordinate within a chunk
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalPos {
    pub x: i32,  // 0-15
    pub y: i32,  // 0-127
    pub z: i32,  // 0-15
}

impl LocalPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        assert!((0..16).contains(&x), "Local X coordinate out of range: {}", x);
        assert!((0..128).contains(&y), "Local Y coordinate out of range: {}", y);
        assert!((0..16).contains(&z), "Local Z coordinate out of range: {}", z);
        Self { x, y, z }
    }
    
    /// Convert local coordinates to array index for chunk storage
    /// 
    /// This uses the same indexing as the original Java code:
    /// index = x << 11 | z << 7 | y
    pub fn to_index(&self) -> usize {
        ((self.x << 11) | (self.z << 7) | self.y) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_packing() {
        let pos = Position::new(100, 200, 300);
        assert_eq!(pos.x(), 100);
        assert_eq!(pos.y(), 200);
        assert_eq!(pos.z(), 300);
    }

    #[test]
    fn test_position_from_packed() {
        let pos1 = Position::new(100, 200, 300);
        let pos2 = Position::from_packed(pos1.packed());
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_position_distance() {
        let pos1 = Position::new(0, 0, 0);
        let pos2 = Position::new(5, 10, 15);
        assert_eq!(pos1.distance_to(&pos2), 30);
    }

    #[test]
    fn test_world_to_chunk_pos() {
        let world_pos = WorldPos::new(32, 64, 48);
        let chunk_pos = world_pos.to_chunk_pos();
        assert_eq!(chunk_pos.x, 2);
        assert_eq!(chunk_pos.z, 3);
    }

    #[test]
    fn test_world_to_local_pos() {
        let world_pos = WorldPos::new(35, 64, 51);
        let local_pos = world_pos.to_local_pos();
        assert_eq!(local_pos.x, 3);
        assert_eq!(local_pos.y, 64);
        assert_eq!(local_pos.z, 3);
    }

    #[test]
    fn test_local_to_index() {
        let local_pos = LocalPos::new(5, 10, 15);
        let index = local_pos.to_index();
        assert_eq!(index, (5 << 11) | (15 << 7) | 10);
    }
}
