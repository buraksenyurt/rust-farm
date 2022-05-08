use crate::game_state::GameState;

// Oyun state nesnesidir. Oyunun hangi konumda olduğunu ve oyuncunun toplam hareket sayısını tutar.
#[derive(Default)]
pub struct GamePlay {
    pub state: GameState,
    pub moves_count: u32,
}
