use bevy::prelude::*;

use crate::entity::defines::TileState;


#[derive(Component, Debug, Clone, PartialEq)]
pub struct Tile {
    pub value: u32,
    pub position_x: i32,
    pub position_y: i32,
    pub state: TileState,
}

impl Tile {
    pub fn new(value: u32, position_x: i32, position_y: i32) -> Self {
        Tile {
            value,
            position_x,
            position_y,
            state: TileState::Spawned,
        }
    }

    pub fn merge(&mut self, value: u32) {
        self.value += value;
        self.state = TileState::Merged;
    }
}