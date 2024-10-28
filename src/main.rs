use bevy::prelude::*;
use ts1_game_2048_rust::{system, ui};
use ts1_game_2048_rust::component::board::Board;
use ts1_game_2048_rust::system::resource::GameContext;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.88, 0.88, 0.88)))
        .insert_resource(Board::create_add_random_tiles())
        .insert_resource(GameContext::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            // 기본 해상도 설정
            primary_window: Some(Window {
                title: "2047!".to_string(),
                resolution: (400.0, 400.0).into(),
                ..default()
            }),
            ..Default::default()
        }))
        .add_event::<system::events::ChangeGameStatus>()
        .add_event::<system::events::MoveTiles>()
        .add_systems(Startup, (system::camera::spawn_camera))
        .add_systems(Update,
            (
                system::handle_keyboard_input::handle_keyboard_input,
                system::window_util::handle_window_close,
            ),
        )
        .add_systems(Update,
            (
                ui::show_menu::show_menu.after(
                    system::handle_keyboard_input::handle_keyboard_input),
                system::game::move_tile.after(
                    system::handle_keyboard_input::handle_keyboard_input),
            ),
        )
        .run();
}
