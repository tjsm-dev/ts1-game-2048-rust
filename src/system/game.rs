use bevy::prelude::*;

use crate::component::board::Board;
use crate::system::events::MoveTiles;
use crate::common::direction::Direction;

pub fn move_tile(
    mut board: ResMut<Board>,
    mut move_tiles: EventReader<MoveTiles>,
) {
    for event in move_tiles.read() {
        match event.direction {
            Direction::Up => {
                println!("Move Up");
            }
            Direction::Down => {
                println!("Move Down");
            }
            Direction::Left => {
                println!("Move Left");
                board.move_left();
                board.spawn_tiles();
            }
            Direction::Right => {
                println!("Move Right");
            }
            Direction::None => {
                /*let b = boards.iter()
                    .map(|e| *board.get_mut(*e).unwrap())
                    .collect::<Vec<Board>>();
                for mut board in b {
                    board.print();
                }*/
            }
        }
    }
}