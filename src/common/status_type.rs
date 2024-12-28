#[derive(PartialEq, Debug)]
pub enum ViewStatusType {
    Rank,
    Game,
}

impl Default for ViewStatusType {
    fn default() -> Self {
        ViewStatusType::Game
    }
}

#[derive(PartialEq)]
pub enum GameStatusType {
    Ready,
    OnGame,
    GameOver,
}

impl Default for GameStatusType {
    fn default() -> Self {
        GameStatusType::OnGame
    }
}