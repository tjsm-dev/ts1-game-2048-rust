use bevy::prelude::*;

pub enum StatusType {
    MainMenu,
    Rank,
    Game,
}

#[derive(Event)]
pub struct ChangeGameStatus(pub StatusType);


#[derive(Debug, Event, Clone, Reflect, Default)]
#[reflect(Debug, Default)]
pub struct MoveTiles;