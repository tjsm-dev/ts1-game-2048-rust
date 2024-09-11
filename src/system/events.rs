use bevy::prelude::*;

use crate::common::{status_type::GameStatusType, direction::Direction};

#[derive(Event)]
pub struct ChangeGameStatus(pub GameStatusType);


#[derive(Debug, Event, Clone)]
pub struct MoveTiles {
    pub direction: Direction,
}

impl Default for MoveTiles {
    fn default() -> Self {
        Self {
            direction: Direction::None,
        }
    }
}