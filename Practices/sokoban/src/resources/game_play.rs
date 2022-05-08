use crate::game_state::GameState;
use std::fmt::{Display, Formatter};

// Oyun state nesnesidir. Oyunun hangi konumda olduğunu ve oyuncunun toplam hareket sayısını tutar.
#[derive(Default)]
pub struct GamePlay {
    pub state: GameState,
    pub moves_count: u32,
}

impl Display for GamePlay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.state {
            GameState::Win => write!(f, "Oyuncu kazandı"),
            GameState::Play => write!(f, "Oynanıyor"),
        }
    }
}
