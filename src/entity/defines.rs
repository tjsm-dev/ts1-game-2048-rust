pub const TILE_WIDTH: i32 = 4;
pub const TILE_HEIGHT: i32 = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileState {
    Spawned,
    Merged,
    Moved,
}