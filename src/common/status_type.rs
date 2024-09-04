#[derive(PartialEq)]
pub enum GameStatusType {
    MainMenu,
    Rank,
    Game,
}

impl GameStatusType {
    pub fn is_equivalent(&self, other: &Self) -> bool {
        self == other || matches!(self, GameStatusType::MainMenu) || matches!(other, GameStatusType::MainMenu)
    }
}

impl Default for GameStatusType {
    fn default() -> Self {
        GameStatusType::Game
    }
}