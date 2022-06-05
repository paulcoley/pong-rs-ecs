pub trait GameState {
    fn execute(&mut self, delta_time: f32);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GameStates {
    Pong,
    MainMenu,
    PauseMenu
}