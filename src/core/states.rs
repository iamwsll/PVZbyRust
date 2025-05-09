#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Menu,
    InGame,
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Menu
    }
}
