use bevy::prelude::*;

use crate::common::{status_type::ViewStatusType, direction::Direction};
use serde::{Deserialize, Serialize};

#[derive(Event)]
pub struct ChangeGameStatus(pub ViewStatusType);


#[derive(Debug, Event, Clone)]
// #[reflect(Debug)]
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

// Add these struct definitions
#[derive(Event)]
pub struct TextPopup;

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

pub struct TextPopupExpires {
    pub expires_in: f32,
}
