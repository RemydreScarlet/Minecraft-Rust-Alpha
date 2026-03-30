//! Mob entity implementation
//! 
//! This module implements mob entities.

use crate::entities::entity::Entity;

pub struct Mob {
    pub entity: Entity,
}

impl Mob {
    pub fn update(&mut self) {
        self.entity.update();
    }
}
