use bevy::prelude::*;

use super::events::{StatusType, ChangeGameStatus};

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<ChangeGameStatus>
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
        events.send(ChangeGameStatus(StatusType::MainMenu));
    }
}