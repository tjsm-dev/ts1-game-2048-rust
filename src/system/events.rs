use bevy::prelude::*;

pub enum StatusType {
    MainMenu,
    Rank,
    Game,
}

#[derive(Event)]
pub struct ChangeGameStatus(pub StatusType);
