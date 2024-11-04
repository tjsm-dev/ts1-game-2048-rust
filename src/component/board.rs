use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::component::tile::Tile;
use crate::entity::defines::{TILE_HEIGHT, TILE_WIDTH};


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
        let mut tiles : [[Tile; 4]; 4] = [
            [Tile::new(0), Tile::new(0),Tile::new(0),Tile::new(0)],
            [Tile::new(0),Tile::new(0),Tile::new(0),Tile::new(0)],
            [Tile::new(0),Tile::new(0),Tile::new(0),Tile::new(0)],
            [Tile::new(0),Tile::new(0),Tile::new(0),Tile::new(0)],
        ];
        tiles
    }

    pub fn add_score(&mut self, score: u16) {
        self.score += score;
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

    pub fn spawn_tiles(&mut self) {
        self.spawn_tile();
        self.spawn_tile();
        self.print();
    }

    fn spawn_tile(&mut self) {
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


    pub fn move_left(&mut self) {
        // 결과를 저장할 새로운 보드를 생성한다.
        // y축 기준으로 for문을 돌린다.
        // 0부터 4까지 순차적으로 value를 확인하여, merge가 가능한지 확인한다.
        // merge가 가능하다는 것은 인접한 타일의 value가 같다는 것이다.
        // 처음 발견한 value와 같은 value를 가진 타일이 등장하면 두 타일의 합을 새 보드에 저장한다.
        // 이때, score를 증가시킨다.
        // score는 합친 value들의 합이다.

        println!("Before move_left");
        self.print();

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
        println!("After move_left");
        self.print();
    }
}