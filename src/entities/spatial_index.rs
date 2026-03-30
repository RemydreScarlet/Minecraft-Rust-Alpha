//! Spatial indexing for entities
//! 
//! This module implements spatial partitioning for efficient entity queries.


/// Spatial index for partitioning entities by Y-section
pub struct SpatialIndex {
    /// Entities partitioned by Y-section (16 blocks each)
    sections: [Vec<crate::entities::entity::Entity>; 8],
}

impl SpatialIndex {
    /// Create a new spatial index
    pub fn new() -> Self {
        Self {
            sections: [const { Vec::new() }; 8],
        }
    }
    
    /// Add an entity to the spatial index
    pub fn add_entity(&mut self, entity: &crate::entities::entity::Entity) {
        let section = (entity.y as i32 / 16).clamp(0, 7) as usize;
        self.sections[section].push(*entity);
    }
    
    /// Get entities in a specific Y-section
    pub fn get_entities_in_section(&self, y: i32) -> &[crate::entities::entity::Entity] {
        let section = (y / 16).clamp(0, 7) as usize;
        &self.sections[section]
    }
    
    /// Get all entities
    pub fn get_all_entities(&self) -> Vec<&crate::entities::entity::Entity> {
        let mut all_entities = Vec::new();
        for section in &self.sections {
            for entity in section {
                all_entities.push(entity);
            }
        }
        all_entities
    }
}

impl Default for SpatialIndex {
    fn default() -> Self {
        Self::new()
    }
}
