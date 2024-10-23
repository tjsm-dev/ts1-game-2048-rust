use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::component::tile::Tile;
use crate::entity::defines::{TILE_HEIGHT, TILE_WIDTH};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Board {
    pub tiles: [[Tile; 4]; 4],
    pub score: u16,
}

impl Board {

    pub fn new() -> Self {

        let mut tiles : [[Tile; 4]; 4] = [
            [Tile::new(0), Tile::new(0),Tile::new(0),Tile::new(0)],
            [Tile::new(0),Tile::new(0),Tile::new(0),Tile::new(0)],
            [Tile::new(0),Tile::new(0),Tile::new(0),Tile::new(0)],
            [Tile::new(0),Tile::new(0),Tile::new(0),Tile::new(0)],
        ];

        let mut board = Board {
            tiles,
            score: 0,
        };
        board.spawn_tile();
        board.spawn_tile();
        board
    }

    pub fn copy(&self) -> Self {

        let mut board = Board {
            tiles: self.tiles.clone(),
            score: self.score,
        };
        board
    }

    pub fn print(&self) {
        print!("Board:\n");
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
               print!("{} ", &(self.tiles[y][x]).value);
            }
            println!();
        }
    }

    pub fn spawn_tile(&mut self) {
        let mut rng = thread_rng();
        let empty_positions = self.get_empty_positions();
        let p = empty_positions[rng.gen_range(0..empty_positions.len())];

        // 60% chance of spawning a 2, 40% chance of spawning a 4
        let value = if rng.gen_bool(0.6) { 2 } else { 4 };
        self.tiles[p.y][p.x].value = value;
    }

    pub fn get_empty_positions(&self) -> Vec<Position> {
        let mut empty_positions = Vec::new();
        for x in 0..TILE_WIDTH {
            for y in 0..TILE_HEIGHT {
                if self.tiles[y][x].value == 0 {
                    empty_positions.push(Position { x, y });
                }
            }
        };
        empty_positions
    }

    //pub fn move_tiles(&mut self, direction: &Direction) {
      //  let mut moved = false;
        //let mut merged = false;
        //let mut new_empty_positions = HashSet::new();
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
        // self.empty_positions = new_empty_positions;
   // }
}