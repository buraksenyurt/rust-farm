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
        println!("playing modundan init'e geçilemez.");
        false
    }
    fn playing(&mut self) -> bool {
        println!("playing modundan playing'e geçilemez.");
        false
    }

    fn end_game(&mut self) -> bool {
        println!("playing modundan end_game'e geçilebilir.");
        true
    }

    fn menu(&mut self) -> bool {
        println!("playing modundan menu'ye geçilemez.");
        false
    }

    fn play_again(&mut self) -> bool {
        println!("playing modundan play_again'e geçilemez.");
        false
    }
}
