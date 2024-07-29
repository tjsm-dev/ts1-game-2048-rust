use bevy::prelude::*;
use ts1_game_2048_rust::{system, ui};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_event::<system::events::ChangeGameStatus>()
    .add_systems(Startup,
         system::camera::spawn_camera)
    .add_systems(Update,
        (
            system::handle_keyboard_input::handle_keyboard_input,
            ui::show_menu::show_menu.after(
                system::handle_keyboard_input::handle_keyboard_input),
        ),
    )
    .run();
}
