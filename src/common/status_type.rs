// TODO: refactoring - 화면 상태 타입으로 이름 변경
pub enum ViewStatusType {
    MainMenu,
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