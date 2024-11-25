use bevy::prelude::Resource;

use crate::common::status_type::{ViewStatusType, GameStatusType};
#[derive(Default, Resource)]
pub struct GameContext {
    pub status: ViewStatusType,
    pub lifecycle: GameStatusType,
}
