// TODO: refactoring - 화면 상태 타입으로 이름 변경
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

pub enum GameLifecycle {
    Ready,
    OnGame,
    GameOver,
}

impl Default for GameLifecycle {
    fn default() -> Self {
        GameLifecycle::OnGame
    }
}