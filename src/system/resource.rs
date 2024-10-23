use bevy::prelude::{Deref, DerefMut, Entity, Resource};

use crate::common::status_type::GameStatusType;
#[derive(Default, Resource, Deref, DerefMut)]
pub struct GameContext {
    pub status: GameStatusType,
}

// Window ID를 저장하기 위한 Resource 정의
#[derive(Resource)]
pub struct RankWindowId(pub Entity);