use bevy::prelude::*;
use rand::random;

use crate::component::tile::Tile;
use crate::entity::defines::{TILE_HEIGHT, TILE_WIDTH};


#[derive(Component, Debug, Clone, PartialEq)]
pub struct Board {
    pub tiles: Vec<Tile>,
    pub score: u32,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            tiles: Vec::<Tile>::new(),
            score: 0,
        };
        board.spawn_tile();
        board.spawn_tile();
        board
    }

    pub fn spawn_tile(&mut self) {
        if self.tiles.len() >= (TILE_WIDTH * TILE_HEIGHT) as usize {
            return;
        }
        loop {
            let x = (random::<u32>() % TILE_WIDTH as u32) as i32;
            let y = (random::<u32>() % TILE_HEIGHT as u32) as i32;

            if self.get_tile(x, y).is_none() {
                let score = if random::<u32>() % 10 == 0 {
                    4
                } else {
                    2
                };
                self.tiles.push(Tile::new(score, x, y));
                break;
            }
        }
    }

    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.iter().find(|tile| tile.position_x == x && tile.position_y == y)
    }
}