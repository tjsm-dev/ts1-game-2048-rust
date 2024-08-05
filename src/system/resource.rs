use bevy::prelude::{Deref, DerefMut, Entity, Resource};

use crate::common::status_type::GameStatusType;
#[derive(Default, Resource, Deref, DerefMut)]
pub struct GameContext {
    pub status: GameStatusType,
}
#[derive(Default, Resource, Deref, DerefMut)]
pub struct Boards(pub Vec<Entity>);