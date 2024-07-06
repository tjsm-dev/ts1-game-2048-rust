use bevy::prelude::*;
use ts1_game_2048_rust::system;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,
         system::camera::spawn_camera)
    .add_systems(Update, 
        system::handle_keyboard_input::handle_keyboard_input)
    .run();
}
