use bevy::prelude::*;

use crate::component::board::Board;
use crate::system::events::{MoveTiles, UpdateGameStatus, ShowScoreBoard, ToggleScoreBoard};
use crate::ui::score_board::ScoreBoardState;
use crate::common::direction::Direction;
use crate::ui::score_board::save_score;

pub fn move_tile(
    mut board: ResMut<Board>,
    mut move_tiles: EventReader<MoveTiles>,
    mut update_game_status: EventWriter<UpdateGameStatus>,
) {
    for event in move_tiles.read() {
        match event.direction {
            Direction::Up => {
                board.move_up();
                update_game_status.send(UpdateGameStatus);
            }
            Direction::Down => {
                board.move_down();
                update_game_status.send(UpdateGameStatus);
            }
            Direction::Left => {
                board.move_left();
                update_game_status.send(UpdateGameStatus);
            }
            Direction::Right => {
                board.move_right();
                update_game_status.send(UpdateGameStatus);
            }
            Direction::None => {
            }
        }
    }
}

pub fn update_game(
    mut board: ResMut<Board>,
    mut update_game_status: EventReader<UpdateGameStatus>,
    mut show_score_board_event: EventWriter<ShowScoreBoard>,
    mut toggle_score_board_event: EventWriter<ToggleScoreBoard>,
    mut score_board_state: ResMut<ScoreBoardState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for _ in update_game_status.read() {
        board.spawn_tiles();

        if !board.is_moveable() {
            println!("Game Over! Score: {}", board.score);

            // prevent new game until new game started with esc key
            // Save the score before showing the score board
            if !score_board_state.visible {
                if let Err(e) = save_score(board.score as u32) {
                    println!("Failed to save score: {}", e);
                }
                show_score_board_event.send(ShowScoreBoard);
                *board = Board::create_add_random_tiles();
                *score_board_state = ScoreBoardState::default();
            }
        }
    }
}
