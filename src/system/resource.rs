use bevy::prelude::{Entity, Resource};

use crate::common::status_type::{GameStatusType, GameLifecycle};
#[derive(Default, Resource)]
pub struct GameContext {
    pub status: GameStatusType,
    pub lifecycle: GameLifecycle,
}

// Window ID를 저장하기 위한 Resource 정의
#[derive(Resource)]
pub struct RankWindowId(pub Entity);