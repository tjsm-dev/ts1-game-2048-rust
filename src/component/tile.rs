use bevy::prelude::*;

use crate::entity::defines::TileState;


#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Tile {
    pub value: u16,
    pub state: TileState,
}

impl Tile {
    pub fn new(value: u16) -> Self {
        Tile {
            value,
            state: TileState::Spawned,
        }
    }

    pub fn merge(&mut self, value: u16) {
        self.value += value;
        self.state = TileState::Merged;
    }
}