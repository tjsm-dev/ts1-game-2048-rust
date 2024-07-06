use bevy::prelude::*;

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        println!("Move Up");
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        println!("Move Down");
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        println!("Move Left");
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        println!("Move Right");
    }
}