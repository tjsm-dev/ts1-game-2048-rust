use bevy::prelude::*;

use crate::common::status_type::{GameStatusType, ViewStatusType};
use crate::common::direction;
use super::events::{ChangeGameStatus, MoveTiles};
use super::resource::GameContext;

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_context: ResMut<GameContext>,
    mut system_event: EventWriter<ChangeGameStatus>,
    mut game_event: EventWriter<MoveTiles>,
) {
    if game_context.lifecycle == GameStatusType::OnGame {
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            game_event.send(MoveTiles {
                direction: direction::Direction::Up,
            });
    
        } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            game_event.send(MoveTiles {
                direction: direction::Direction::Down,
            });
    
        } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            game_event.send(MoveTiles {
                direction: direction::Direction::Left,
            });
    
        } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            game_event.send(MoveTiles {
                direction: direction::Direction::Right,
            });
        }
    }
    
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        system_event.send(ChangeGameStatus(ViewStatusType::Rank));
    } else if keyboard_input.just_pressed(KeyCode::KeyQ) {

    }
}