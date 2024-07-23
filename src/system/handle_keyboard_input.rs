use bevy::prelude::*;

use super::events::{MenuType, ShowMenu};
use super::events::{ScoreBoardType, ShowScoreBoard};

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<ShowMenu>,
    mut events_score_board: EventWriter<ShowScoreBoard>
) {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        println!("Move Up");
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        println!("Move Down");
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        println!("Move Left");
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        println!("Move Right");
    } else if keyboard_input.pressed(KeyCode::Escape) {
        events.send(ShowMenu(MenuType::Main));
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        events_score_board.send(ShowScoreBoard(ScoreBoardType::User));
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        events_score_board.send(ShowScoreBoard(ScoreBoardType::Score));
    }
}