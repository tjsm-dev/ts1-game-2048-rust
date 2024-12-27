use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::system::events::{ChangeGameStatus, ShowScoreBoard};
use crate::common::status_type::ViewStatusType;
use crate::system::resource::GameContext;

const MAIN_MENU_WIDTH: f32 = 200.;
const MAIN_MENU_HEIGHT: f32 = 200.;

#[derive(Component)]
struct MainMenuHandle;

pub fn show_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<ChangeGameStatus>,
    mut game_context: ResMut<GameContext>,
    mut score_board_event: EventWriter<ShowScoreBoard>
) {
    for event in events.read() {
        match event.0 {
            ViewStatusType::Rank => {
                score_board_event.send(ShowScoreBoard);
            }
            ViewStatusType::Game => {
                println!("Game Menu Selected!");
                game_context.status = ViewStatusType::Game;
            }
        }
    }
}
