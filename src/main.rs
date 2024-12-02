use bevy::prelude::*;
use ts1_game_2048_rust::{system, ui};
use ts1_game_2048_rust::component::board::Board;
use ts1_game_2048_rust::system::resource::GameContext;
use crate::system::events::{ShowScoreBoard, ToggleScoreBoard};
use crate::ui::score_board::{create_score_board, handle_score_board_input, ScoreBoardState};
use ts1_game_2048_rust::ui::game_ui::{spawn_game_ui, sync_board_with_ui};
use bevy::window::WindowResolution;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.88, 0.88, 0.88)))
        .insert_resource(Board::create_add_random_tiles())
        .insert_resource(GameContext::default())
        .init_resource::<ScoreBoardState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2047!".to_string(),
                resolution: WindowResolution::new(800.0, 600.0),
                resizable: true,
                ..default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (
            system::camera::spawn_camera,
            spawn_game_ui,
        ))
        .add_systems(Update, (
            system::handle_keyboard_input::handle_keyboard_input,
            ui::show_menu::show_menu.after(system::handle_keyboard_input::handle_keyboard_input),
            system::game::move_tile.after(system::handle_keyboard_input::handle_keyboard_input),
            system::game::update_game.after(system::game::move_tile),
            create_score_board.after(system::handle_keyboard_input::handle_keyboard_input),
            handle_score_board_input,
            sync_board_with_ui.after(system::game::update_game),
            ui::game_ui::animate_tiles,
            ui::game_ui::animate_merges,
        ))
        .add_event::<system::events::ChangeGameStatus>()
        .add_event::<system::events::MoveTiles>()
        .add_event::<system::events::UpdateGameStatus>()
        .add_event::<ShowScoreBoard>()
        .add_event::<ToggleScoreBoard>()
        .run();
}
