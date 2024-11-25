use bevy::prelude::*;

use crate::common::{status_type::ViewStatusType, direction::Direction};
use serde::{Deserialize, Serialize};

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

#[derive(Event)]
pub struct TextPopupEvent {
    pub content: String,
    pub font: Option<Handle<Font>>,
    pub font_size: f32,
    pub font_color: Color,
    pub margin: UiRect,
    pub padding: UiRect,
}

pub enum ScoreBoardType {
    Current,
    HighScore,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreData {
    pub score: u32,
    pub date: String,
}

#[derive(Debug, Event)]
pub struct ShowScoreBoard;
