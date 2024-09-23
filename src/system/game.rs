use bevy::prelude::*;

use crate::entity::defines::Boards;
use crate::component::board::Board;
use crate::system::{events::MoveTiles, resource::GameContext};

pub fn spawn_board(mut commands: Commands, mut boards: ResMut<Boards>) {
    println!("Spawning board");
    let mut board = Board::new();
    board.print();
    *boards = Boards(vec![
        commands
            .spawn_empty()
            .insert(board)
            .id()
    ]);
}

pub fn move_tile(
    mut commands: Commands,
    mut board: Query<&mut Board>,
    mut boards: ResMut<Boards>,
    mut move_tiles: EventReader<MoveTiles>,
    mut game_context: ResMut<GameContext>
) {
    for event in move_tiles.read() {
        match event.direction {
            _ => {
                println!("Move Tile");
                let b = boards.iter()
                    .map(|e| *board.get_mut(*e).unwrap())
                    .collect::<Vec<Board>>();
                for mut board in b {
                    board.print();
                }
            }
        }
    }
}