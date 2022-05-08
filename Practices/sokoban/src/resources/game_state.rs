use std::fmt::{Display, Formatter};

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

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::Play => write!(f, "Oyun modu"),
            GameState::Win => write!(f, "Kazanıldı"),
        }
    }
}
