use std::collections::HashSet;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::component::tile::Tile;
use crate::entity::defines::{TILE_HEIGHT, TILE_WIDTH};

use crate::system::events::MoveTiles;


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

    pub fn print(&self) {
        print!("Board:\n");
        for y in 0..TILE_HEIGHT {
            for x in 0..TILE_WIDTH {
                let tile = self.get_tile(x, y);
                if tile.is_none() {
                    print!("0 ");
                } else {
                    print!("{} ", tile.unwrap().value);
                }
            }
            println!();
        }
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

    pub fn tiles_movement_event(
        &mut self,
        mut move_event: EventReader<MoveTiles>
    ) {
        for _ in move_event.read() {
            // self.move_tiles();
        }
    }

    pub fn move_tiles(&mut self, direction: &Direction) {
        let mut moved = false;
        let mut merged = false;
        let mut new_empty_positions = HashSet::new();
        // for x in 0..TILE_WIDTH {
        //     for y in 0..TILE_HEIGHT {
        //         let mut tile = self.get_tile(x, y).cloned();
        //         if tile.is_none() {
        //             continue;
        //         }
        //         let mut tile = tile.unwrap();
        //         let mut new_x = x;
        //         let mut new_y = y;
        //         let mut new_tile = tile.clone();
        //         while {
        //             match direction {
        //                 Direction::Up => {
        //                     new_y = new_y.saturating_sub(1);
        //                 }
        //                 Direction::Down => {
        //                     new_y = new_y.saturating_add(1);
        //                 }
        //                 Direction::Left => {
        //                     new_x = new_x.saturating_sub(1);
        //                 }
        //                 Direction::Right => {
        //                     new_x = new_x.saturating_add(1);
        //                 }
        //             }
        //             if new_x >= TILE_WIDTH || new_y >= TILE_HEIGHT {
        //                 break;
        //             }
        //             let new_tile = self.get_tile(new_x, new_y).cloned();
        //             if new_tile.is_none() {
        //                 continue;
        //             }
        //             let new_tile = new_tile.unwrap();
        //             if new_tile.value == tile.value && !merged {
        //                 tile.merge(new_tile.value);
        //                 self.tiles.retain(|t| t.position_x != new_x || t.position_y != new_y);
        //                 merged = true;
        //                 moved = true;
        //             }
        //             break;
        //         } {}
        //         if new_x != x || new_y != y {
        //             self.tiles.retain(|t| t.position_x != x || t.position_y != y);
        //             new_tile.position_x = new_x;
        //             new_tile.position_y = new_y;
        //             self.tiles.push(new_tile);
        //             moved = true;
        //         }
        //         new_empty_positions.insert((new_x, new_y));
        //     }
        // }
        self.empty_positions = new_empty_positions;
    }
}