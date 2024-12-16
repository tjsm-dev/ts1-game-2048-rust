use bevy::prelude::*;

use crate::common::{status_type::ViewStatusType, direction::Direction};

#[derive(Event)]
pub struct ChangeGameStatus(pub ViewStatusType);


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

#[derive(Debug, Event)]
pub struct UpdateGameStatus;

pub enum ScoreBoardType {
    Current,
    HighScore,
}

#[derive(Debug, Event)]
pub struct ShowScoreBoard;

#[derive(Debug, Event)]
pub struct ToggleScoreBoard;