use bevy::prelude::*;

use crate::common::status_type::ViewStatusType;
use crate::common::direction;
use super::events::{ChangeGameStatus, MoveTiles, ShowScoreBoard};

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut system_event: EventWriter<ChangeGameStatus>,
    mut game_event: EventWriter<MoveTiles>,
    mut score_board_event: EventWriter<ShowScoreBoard>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        game_event.send(MoveTiles {
            direction: direction::Direction::Up,
        });

    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        game_event.send(MoveTiles {
            direction: direction::Direction::Down,
        });

    } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        game_event.send(MoveTiles {
            direction: direction::Direction::Left,
        });

    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        game_event.send(MoveTiles {
            direction: direction::Direction::Right,
        });
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        system_event.send(ChangeGameStatus(ViewStatusType::MainMenu));
    } else if keyboard_input.just_pressed(KeyCode::KeyR) {
        system_event.send(ChangeGameStatus(ViewStatusType::Rank));
    } else if keyboard_input.just_pressed(KeyCode::KeyQ) {

    }
    // TODO: Add logic for score board
    else if keyboard_input.just_pressed(KeyCode::KeyS) {
        score_board_event.send(ShowScoreBoard);
    }
}