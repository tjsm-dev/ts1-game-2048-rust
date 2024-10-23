use bevy::prelude::*;


#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Tile {
    pub value: u16,
}

impl Tile {
    pub fn new(value: u16) -> Self {
        Tile {
            value,
        }
    }

    pub fn merge(&mut self, value: u16) {
        self.value += value;
    }
}