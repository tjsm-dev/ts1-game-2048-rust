use bevy::prelude::*;
use ts1_game_2048_rust::{system, ui};
use ts1_game_2048_rust::entity::defines::Boards;
use ts1_game_2048_rust::system::resource::GameContext;

fn main() {
    App::new()
        .insert_resource(Boards::default())
        .insert_resource(GameContext::default())
        .add_plugins(DefaultPlugins)
        .add_event::<system::events::ChangeGameStatus>()
        .add_event::<system::events::MoveTiles>()
        .add_systems(Startup, (system::camera::spawn_camera, system::game::spawn_board))
        .add_systems(Update,
            (
                system::handle_keyboard_input::handle_keyboard_input,
                ui::show_menu::show_menu.after(
                    system::handle_keyboard_input::handle_keyboard_input),
                system::window_util::handle_window_close,
            ),
        )
        .run();
}
