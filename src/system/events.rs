use bevy::prelude::*;

pub enum MenuType {
    Main,
    Rank
}

#[derive(Event)]
pub struct ShowMenu(pub MenuType);
