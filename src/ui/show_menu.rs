use bevy::prelude::*;

use crate::system::events::{ChangeGameStatus, ShowScoreBoard};
use crate::common::status_type::ViewStatusType;
use crate::system::resource::GameContext;

const MAIN_MENU_WIDTH: f32 = 200.;
const MAIN_MENU_HEIGHT: f32 = 200.;

#[derive(Component)]
struct MainMenuHandle;

pub fn show_menu(
    mut events: EventReader<ChangeGameStatus>,
    mut game_context: ResMut<GameContext>,
    mut score_board_event: EventWriter<ShowScoreBoard>
) {
    for event in events.read() {
        match event.0 {
            ViewStatusType::Rank => {
                game_context.status = ViewStatusType::Rank;
                score_board_event.send(ShowScoreBoard);
            }
            ViewStatusType::Game => {
                game_context.status = ViewStatusType::Game;
                score_board_event.send(ShowScoreBoard);
            }
        }
    }
}
