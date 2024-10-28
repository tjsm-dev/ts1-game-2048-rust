use bevy::prelude::*;
use bevy::input::ButtonInput;
use ts1_game_2048_rust::{system, ui};
use ts1_game_2048_rust::entity::defines::Boards;
use ts1_game_2048_rust::system::resource::GameContext;
use crate::system::events::{TextPopup, ShowScoreBoard, TextPopupEvent};
use crate::ui::score_board::{TextPopupExpires, create_score_board, cleanup_score_board, ScoreBoardState};
use bevy::input::keyboard::KeyCode;

fn main() {
    App::new()
        .insert_resource(Boards::default())
        .insert_resource(GameContext::default())
        .insert_resource(ScoreBoardState::default())
        .add_plugins(DefaultPlugins)
        .add_event::<system::events::ChangeGameStatus>()
        .add_event::<system::events::MoveTiles>()
        .add_systems(Startup, (system::camera::spawn_camera, system::game::spawn_board))
        // Menu systems - triggered by Escape
        .add_systems(
            Update,
            (
                system::handle_keyboard_input::handle_keyboard_input,
                ui::show_menu::show_menu,
            ).run_if(|keyboard: Res<ButtonInput<KeyCode>>| keyboard.just_pressed(KeyCode::Escape))
        )
        // Score board systems - triggered by S key
        .add_systems(
            Update,
            (
                create_score_board,
            ).run_if(|keyboard: Res<ButtonInput<KeyCode>>| keyboard.just_pressed(KeyCode::KeyS))
        )
        .add_event::<TextPopup>()
        .add_event::<ShowScoreBoard>()
        .add_event::<TextPopupExpires>()
        .add_event::<TextPopupEvent>()
        .run();
}
