use bevy::prelude::*;

use crate::common::status_type::GameStatusType;
use super::events::{ChangeGameStatus, MoveTiles, ShowScoreBoard};
use crate::common::direction;
use crate::system::events::ScoreBoardType;

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut system_event: EventWriter<ChangeGameStatus>,
    mut game_event: EventWriter<MoveTiles>,
    mut score_board_event: EventWriter<ShowScoreBoard>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        println!("Move Up");

    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        println!("Move Down");

    } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        println!("Move Left");

    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        println!("Move Right");
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        system_event.send(ChangeGameStatus(GameStatusType::MainMenu));
    } else if keyboard_input.just_pressed(KeyCode::KeyQ) {
        game_event.send(MoveTiles {
            direction: direction::Direction::Up,
        });
        
    }
    // TODO: Add logic for score board
    else if keyboard_input.just_pressed(KeyCode::KeyS) {
        score_board_event.send(ShowScoreBoard);
        println!("Show Score Board");
    }
}
