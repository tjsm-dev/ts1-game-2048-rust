use std::collections::HashSet;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::component::tile::Tile;
use crate::entity::defines::{TILE_HEIGHT, TILE_WIDTH};


#[derive(Component, Debug, Clone, PartialEq)]
pub struct Board {
    pub tiles: Vec<Tile>,
    pub score: u16,
    empty_positions: HashSet<(u8, u8)>,
}

impl Board {
    pub fn new() -> Self {
        let mut empty_positions = HashSet::new();
        for x in 0..TILE_WIDTH {
            for y in 0..TILE_HEIGHT {
                empty_positions.insert((x, y));
            }
        }
        let mut board = Board {
            tiles: Vec::<Tile>::new(),
            score: 0,
            empty_positions,
        };
        board.spawn_tile();
        board.spawn_tile();
        board
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.empty_positions.remove(&(tile.position_x, tile.position_y));
        self.tiles.push(tile);
    }

    pub fn spawn_tile(&mut self) -> Option<&Tile> {
        if self.empty_positions.is_empty() {
            return None;
        }

        let mut rng = thread_rng();
        let empty_positions: Vec<&(u8, u8)> = self.empty_positions.iter().collect();
        let &(x, y) = empty_positions[rng.gen_range(0..empty_positions.len())];

        // 60% chance of spawning a 2, 40% chance of spawning a 4
        let value = if rng.gen_bool(0.6) { 2 } else { 4 };

        let tile = Tile::new(value, x, y);
        self.add_tile(tile);
        self.tiles.last()
    }

    pub fn get_tile(&self, x: u8, y: u8) -> Option<&Tile> {
        self.tiles.iter().find(|t| t.position_x == x && t.position_y == y)
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn count_tiles(&self) -> usize {
        self.tiles.len()
    }

    pub fn get_empty_positions(&self) -> &HashSet<(u8, u8)> {
        &self.empty_positions
    }
}
