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

#[derive(Debug, Event)]
pub struct ShowScoreBoard;
