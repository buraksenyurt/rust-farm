use crate::game_state::GameState;
use crate::state::State;

pub struct PlayAgainState {
    pub name: String,
}

impl State for PlayAgainState {
    fn get_state(&self) -> String {
        self.name.clone()
    }
}

// Oyun kaybedildiğinde yeniden oynanmak istenirse geçilen moddur.
// PlayAgain durumundan EndGame durumuna geçilebilir.
impl GameState for PlayAgainState {
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
