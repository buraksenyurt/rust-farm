use crate::game_state::GameState;
use crate::state::State;

// Playing durumunu temsil eden veri yapısı
pub struct PlayingState {
    pub name: String,
}

impl State for PlayingState {
    fn get_state(&self) -> String {
        self.name.clone()
    }
}

// Oyun oynanırken Playing modundayız.
// Oyun kaybedildiğinde bu durumdan EndGame durumuna geçilebilir.
impl GameState for PlayingState {
    fn init(&mut self) -> bool {
        false
    }
    fn playing(&mut self) -> bool {
        false
    }

    fn end_game(&mut self) -> bool {
        true
    }

    fn menu(&mut self) -> bool {
        false
    }

    fn play_again(&mut self) -> bool {
        false
    }
}
