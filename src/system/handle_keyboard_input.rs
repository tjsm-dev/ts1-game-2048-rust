use bevy::prelude::*;

use super::events::{MenuType, ShowMenu};

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<ShowMenu>
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
    }
}