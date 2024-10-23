use bevy::prelude::*;

pub const TILE_WIDTH: usize = 4;
pub const TILE_HEIGHT: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileState {
    Spawned,
    Merged,
    Moved,
}

#[derive(Default, Resource, Deref, DerefMut)]
pub struct Boards(pub Vec<Entity>);