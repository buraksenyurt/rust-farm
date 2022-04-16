use crate::game_state::GameState;
use crate::state::State;

// EndGame durumunu temsil eden veri yapısı
pub struct EndGameState {
    pub name: String,
}

impl State for EndGameState {
    fn get_state(&self) -> String {
        self.name.clone()
    }
}

// Oyun kaybedildiğinde geçilen EndGame durumudur.
// Oyun sonlandığında menüye dönülebilir ya da yeniden oynamaya geçilebilir.
impl GameState for EndGameState {
    fn init(&mut self) -> bool {
        false
    }
    fn playing(&mut self) -> bool {
        false
    }

    fn end_game(&mut self) -> bool {
        false
    }

    fn menu(&mut self) -> bool {
        true
    }

    fn play_again(&mut self) -> bool {
        true
    }
}
