//! Player entity implementation
//! 
//! This module implements the player entity.

use crate::entities::entity::Entity;

#[derive(Clone)]
pub struct Player {
    pub entity: Entity,
    pub health: i32,
}

impl Player {
    pub fn update(&mut self) {
        self.entity.update();
    }
}
