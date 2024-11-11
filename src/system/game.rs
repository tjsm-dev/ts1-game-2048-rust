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
                board.move_up();
                board.spawn_tiles();
            }
            Direction::Down => {
                board.move_down();
                board.spawn_tiles();
            }
            Direction::Left => {
                board.move_left();
                board.spawn_tiles();
            }
            Direction::Right => {
                board.move_right();
                board.spawn_tiles();
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