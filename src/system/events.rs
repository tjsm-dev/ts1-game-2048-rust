use bevy::prelude::*;

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