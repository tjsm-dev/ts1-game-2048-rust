pub const TILE_WIDTH: u8 = 4;
pub const TILE_HEIGHT: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileState {
    Spawned,
    Merged,
    Moved,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
    None,
}