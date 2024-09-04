use bevy::prelude::*;

<<<<<<< Updated upstream
pub enum MenuType {
    Main,
    Rank
}

pub enum ScoreBoardType {
    User,
    Score,
    Rank
}

#[derive(Event)]
pub struct ShowMenu(pub MenuType);

#[derive(Event)]
pub struct ShowScoreBoard(pub ScoreBoardType);

pub struct TextPopupEvent {
    pub content: String,
}

#[derive(Debug, Component)]
pub struct TextPopup;
=======
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

#[derive(Event)]
pub struct ShowScoreBoard;

#[derive(Debug, Event, Clone)]
pub struct ScoreBoardType;

#[derive(Event)]
pub struct TextPopupEvent {
    pub content: String,
    pub font: Option<Handle<Font>>,
    pub font_size: f32,
    pub font_color: Color,
    pub border: UiRect,
    pub border_color: Color,
    pub padding: UiRect,
    pub margin: UiRect,
    pub modal: Option<bool>,
    pub text_alignment: JustifyText,
    pub background_color: Color,
}

#[derive(Component)]
pub struct TextPopupExpires {
    pub expires_in: f32,
}
>>>>>>> Stashed changes
