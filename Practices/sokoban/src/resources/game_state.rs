pub enum GameState {
    Play,
    Win,
}

// GamePlay nesnesi Default trait'ini uyguladığı için eklenmiştir.
impl Default for GameState {
    fn default() -> Self {
        Self::Play
    }
}
