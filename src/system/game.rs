use bevy::prelude::*;
use bevy::utils::petgraph::matrix_graph::Zero;
use crate::common::status_type::GameStatusType;
use crate::component::board::Board;
use crate::system::events::{MoveTiles, UpdateGameStatus, ChangeGameStatus, RestartGame};
use crate::common::direction::Direction;
use crate::system::data::load_scores;
use crate::system::resource::GameContext;
use crate::common::status_type::ViewStatusType;
use super::data::save_score;

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
    mut system_event: EventWriter<ChangeGameStatus>,
    mut game_context: ResMut<GameContext>,
) {
    for _ in update_game_status.read() {
        board.spawn_tiles();

        if !board.is_moveable() {
            
            let result = save_score(board.score as u32);
            match result {
               Ok(_) => {
                    print!("score updated to score board\n")
               }
                Err(_) => {
                    print!("Save score failed\n")
                }
            }
            game_context.lifecycle = GameStatusType::GameOver;
            system_event.send(ChangeGameStatus(ViewStatusType::Rank));
        }
    }
}

pub fn restart_game(
    mut board: ResMut<Board>,
    mut restart_event: EventReader<RestartGame>,
    mut system_event: EventWriter<ChangeGameStatus>,
    mut game_context: ResMut<GameContext>,
) {
    if !restart_event.read().count().is_zero() {
        *board = Board::create_add_random_tiles();
        game_context.lifecycle = GameStatusType::OnGame;
        system_event.send(ChangeGameStatus(ViewStatusType::Game));
        load_game_data(game_context);
    }
}

pub fn load_game_data(
    mut game_context: ResMut<GameContext>,
) {
    let mut scores = load_scores().unwrap_or_default();

    scores.sort_by(|a, b| b.score.cmp(&a.score));
    game_context.best_score = scores.first().map(|s| s.score).unwrap_or(0);
}