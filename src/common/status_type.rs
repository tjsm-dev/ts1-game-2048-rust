pub enum GameStatusType {
    MainMenu,
    Rank,
    Game,
}

impl Default for GameStatusType {
    fn default() -> Self {
        GameStatusType::Game
    }
}