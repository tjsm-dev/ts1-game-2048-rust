use bevy::prelude::*;

use crate::entity::defines::TileState;


#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Tile {
    pub value: u16,
    pub position_x: u8,
    pub position_y: u8,
    pub state: TileState,
}

impl Tile {
    pub fn new(value: u16, position_x: u8, position_y: u8) -> Self {
        Tile {
            value,
            position_x,
            position_y,
            state: TileState::Spawned,
        }
    }

    pub fn merge(&mut self, value: u16) {
        self.value += value;
        self.state = TileState::Merged;
    }
}