use bevy::prelude::{Entity, Resource};

use crate::common::status_type::{ViewStatusType, GameStatusType};
#[derive(Default, Resource)]
pub struct GameContext {
    pub status: ViewStatusType,
    pub lifecycle: GameStatusType,
}

// Window ID를 저장하기 위한 Resource 정의
#[derive(Resource)]
pub struct RankWindowId(pub Entity);