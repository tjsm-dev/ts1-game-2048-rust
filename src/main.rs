use bevy::prelude::*;
use ts1_game_2048_rust::{system, ui};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_event::<system::events::ShowMenu>()
    .add_systems(Startup,
         system::camera::spawn_camera)
    .add_systems(Update,
        (
            system::handle_keyboard_input::handle_keyboard_input,
            ui::main_menu::show_main_menu.after(
                system::handle_keyboard_input::handle_keyboard_input),
            system::handle_keyboard_input::move_tiles,
        ),
    )
    .run();
}
