pub const TILE_WIDTH: u8 = 4;
pub const TILE_HEIGHT: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileState {
    Spawned,
    Merged,
    Moved,
}