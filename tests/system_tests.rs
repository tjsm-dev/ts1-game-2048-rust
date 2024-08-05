use bevy::prelude::*;

use ts1_game_2048_rust::system::handle_keyboard_input::handle_keyboard_input;
use ts1_game_2048_rust::entity::defines::Direction;
#[test]
fn test_handle_keyboard_input_up() {
    let mut app = App::new();

    app.add_systems(Update, handle_keyboard_input);

    let mut button_input = ButtonInput::<KeyCode>::default();
    button_input.press(KeyCode::ArrowUp);
    app.insert_resource(button_input);

    app.update();

    println!("{:?}", app.world.get_resource::<ButtonInput<KeyCode>>());

    match app.world.get_resource::<ButtonInput<KeyCode>>() {
        Some(button_input) => {
            assert_eq!(button_input.pressed(KeyCode::ArrowUp), true);
            assert_eq!(button_input.pressed(KeyCode::ArrowDown), false);
            assert_eq!(button_input.pressed(KeyCode::ArrowLeft), false);
            assert_eq!(button_input.pressed(KeyCode::ArrowRight), false);
        }
        None => panic!("ButtonInput resource not found"),
    }
}

#[test]
fn test_handle_keyboard_input_down() {
    let mut app = App::new();

    app.add_systems(Update, handle_keyboard_input);

    let mut button_input = ButtonInput::<KeyCode>::default();
    button_input.press(KeyCode::ArrowDown);
    app.insert_resource(button_input);

    app.update();

    println!("{:?}", app.world.get_resource::<ButtonInput<KeyCode>>());

    match app.world.get_resource::<ButtonInput<KeyCode>>() {
        Some(button_input) => {
            assert_eq!(button_input.pressed(KeyCode::ArrowUp), false);
            assert_eq!(button_input.pressed(KeyCode::ArrowDown), true);
            assert_eq!(button_input.pressed(KeyCode::ArrowLeft), false);
            assert_eq!(button_input.pressed(KeyCode::ArrowRight), false);
        }
        None => panic!("ButtonInput resource not found"),
    }
}