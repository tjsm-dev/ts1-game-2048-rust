use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::component::tile::Tile;
const TILE_WIDTH: usize = 4;
const TILE_HEIGHT: usize = 4;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Resource, Debug, Clone, Copy, PartialEq)]
pub struct Board {
    pub tiles: [[Tile; 4]; 4],
    pub score: u16,
}

impl Board {
    pub fn create_add_random_tiles() -> Self {
        let mut board = Board::create_empty();
        board.spawn_tiles();
        board
    }

    fn create_empty() -> Self {
        Board {
            tiles: Board::create_tiles(),
            score: 0,
        }
    }

    fn create_tiles() -> [[Tile; 4]; 4] {
        [
            [Tile::new(0), Tile::new(0),Tile::new(0),Tile::new(0)],
            [Tile::new(0),Tile::new(0),Tile::new(0),Tile::new(0)],
            [Tile::new(0),Tile::new(0),Tile::new(0),Tile::new(0)],
            [Tile::new(0),Tile::new(0),Tile::new(0),Tile::new(0)],
        ]
    }

    pub fn add_score(&mut self, score: u16) {
        self.score += score;
    }

    pub fn copy(&self) -> Self {
        Board {
            tiles: self.tiles.clone(),
            score: self.score,
        }
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

    fn spawn_tile(&mut self) {
        let mut rng = thread_rng();
        let empty_positions = self.get_empty_positions();
        let p = empty_positions[rng.gen_range(0..empty_positions.len())];

        // 60% chance of spawning a 2, 40% chance of spawning a 4
        let value = if rng.gen_bool(0.6) { 2 } else { 4 };
        self.tiles[p.y][p.x].value = value;
    }

    pub fn spawn_tiles(&mut self) {
        println!("spawn tiles");
        self.spawn_tile();
        self.spawn_tile();
        self.print();
    }

    fn get_empty_positions(&self) -> Vec<Position> {
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

    pub fn move_up(&mut self) {
        let mut new_tiles = Board::create_tiles();

        fn set_value(tiles: &mut [[Tile; 4]; 4], x: usize, value: u16) {
            for y in 0..TILE_HEIGHT {
                if tiles[y][x].value == 0 {
                    tiles[y][x].value = value;
                    break;
                }
            }
        }

        for x in 0..TILE_WIDTH {
            let mut cursor = 0;

            while cursor < TILE_HEIGHT {
                if self.tiles[cursor][x].value == 0 {
                    cursor += 1;
                    continue;
                }

                let mut target_cursor = 0;

                for i in cursor+1..TILE_HEIGHT {
                    if self.tiles[i][x].value == 0 {
                        continue;
                    }
                    target_cursor = i;
                    break;
                }
                if target_cursor == 0 {
                    set_value(&mut new_tiles, x, self.tiles[cursor][x].value);
                    cursor += 1;
                    continue;
                }

                if self.tiles[cursor][x].value == self.tiles[target_cursor][x].value {
                    let merge_value = self.tiles[cursor][x].value * 2;
                    set_value(&mut new_tiles, x, merge_value);
                    self.add_score(merge_value);
                    cursor = target_cursor +1;
                } else {
                    set_value(&mut new_tiles, x, self.tiles[cursor][x].value);
                    cursor = target_cursor;
                }
            }
        }
        self.tiles = new_tiles;
    }

    pub fn move_down(&mut self) {
        let mut new_tiles = Board::create_tiles();

        fn set_value(tiles: &mut [[Tile; 4]; 4], x: usize, value: u16) {
            for y in (0..TILE_HEIGHT).rev() {
                if tiles[y][x].value == 0 {
                    tiles[y][x].value = value;
                    break;
                }
            }
        }

        for x in 0..TILE_WIDTH {
            let mut cursor: isize = (TILE_HEIGHT as isize) - 1;

            while cursor as isize >= 0 {
                if self.tiles[cursor as usize][x].value == 0 {
                    cursor -= 1;
                    continue;
                }

                let mut target_cursor = -1;

                for i in (0..cursor).rev() {
                    if self.tiles[i as usize][x].value == 0 {
                        continue;
                    }
                    target_cursor = i;
                    break;
                }

                if target_cursor == -1 {
                    set_value(&mut new_tiles, x, self.tiles[cursor as usize][x].value);
                    cursor -= 1;
                    continue;
                }

                if self.tiles[cursor as usize][x].value == self.tiles[target_cursor as usize][x].value {
                    let merge_value = self.tiles[cursor as usize][x].value * 2;
                    set_value(&mut new_tiles, x, merge_value);
                    self.add_score(merge_value);
                    cursor = target_cursor -1;
                } else {
                    set_value(&mut new_tiles, x, self.tiles[cursor as usize][x].value);
                    cursor = target_cursor;
                }
            }
        }
        self.tiles = new_tiles;
    }

    pub fn move_left(&mut self) {
        let mut new_tiles = Board::create_tiles();

        fn set_value(tiles: &mut [[Tile; 4]; 4], y: usize, value: u16) {
            for x in 0..TILE_WIDTH {
                if tiles[y][x].value == 0 {
                    tiles[y][x].value = value;
                    break;
                }
            }
        }

        for y in 0..TILE_HEIGHT {
            let mut cursor = 0;

            while cursor < TILE_WIDTH {
                if self.tiles[y][cursor].value == 0 {
                    cursor += 1;
                    continue;
                }

                let mut target_cursor = 0;

                for i in cursor+1..TILE_WIDTH {
                    if self.tiles[y][i].value == 0 {
                        continue;
                    }
                    target_cursor = i;
                    break;
                }
                if target_cursor == 0 {
                    set_value(&mut new_tiles, y, self.tiles[y][cursor].value);
                    cursor += 1;
                    continue;
                }

                if self.tiles[y][cursor].value == self.tiles[y][target_cursor].value {
                    let merge_value = self.tiles[y][cursor].value * 2;
                    set_value(&mut new_tiles, y, merge_value);
                    self.add_score(merge_value);
                    cursor = target_cursor +1;
                } else {
                    set_value(&mut new_tiles, y, self.tiles[y][cursor].value);
                    cursor = target_cursor;
                }
            }
        }
        self.tiles = new_tiles;
    }

    pub fn move_right(&mut self) {
        let mut new_tiles = Board::create_tiles();

        fn set_value(tiles: &mut [[Tile; 4]; 4], y: usize, value: u16) {
            for x in (0..TILE_WIDTH).rev() {
                if tiles[y][x].value == 0 {
                    tiles[y][x].value = value;
                    break;
                }
            }
        }

        for y in 0..TILE_HEIGHT {
            let mut cursor: isize = (TILE_WIDTH as isize) - 1;

            while cursor as isize >= 0 {
                if self.tiles[y][cursor as usize].value == 0 {
                    cursor -= 1;
                    continue;
                }

                let mut target_cursor = -1;

                for i in (0..cursor).rev() {
                    if self.tiles[y][i as usize].value == 0 {
                        continue;
                    }
                    target_cursor = i;
                    break;
                }

                if target_cursor == -1 {
                    set_value(&mut new_tiles, y, self.tiles[y][cursor as usize].value);
                    cursor -= 1;
                    continue;
                }

                if self.tiles[y][cursor as usize].value == self.tiles[y][target_cursor as usize].value {
                    let merge_value = self.tiles[y][cursor as usize].value * 2;
                    set_value(&mut new_tiles, y, merge_value);
                    self.add_score(merge_value);
                    cursor = target_cursor -1;
                } else {
                    set_value(&mut new_tiles, y, self.tiles[y][cursor as usize].value);
                    cursor = target_cursor;
                }
            }
        }
        self.tiles = new_tiles;
    }
}