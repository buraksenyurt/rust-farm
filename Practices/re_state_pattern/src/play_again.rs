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
        println!("play_again modundan init'e geçilemez.");
        false
    }
    fn playing(&mut self) -> bool {
        println!("play_again modundan playing'e geçilemez.");
        false
    }

    fn end_game(&mut self) -> bool {
        println!("play_again modundan end_game'e geçilebilir.");
        true
    }

    fn menu(&mut self) -> bool {
        println!("play_again modundan menu'ye geçilemez.");
        false
    }

    fn play_again(&mut self) -> bool {
        println!("play_again modundan play_again'e geçilemez.");
        false
    }
}
