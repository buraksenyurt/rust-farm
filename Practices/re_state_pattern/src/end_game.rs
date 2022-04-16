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
        println!("end_game modundan init'e geçilemez.");
        false
    }
    fn playing(&mut self) -> bool {
        println!("end_game modundan playing'e geçilemez.");
        false
    }

    fn end_game(&mut self) -> bool {
        println!("end_game modundan end_game'e geçilemez.");
        false
    }

    fn menu(&mut self) -> bool {
        println!("end_game modundan menu'ye geçilebilir.");
        true
    }

    fn play_again(&mut self) -> bool {
        println!("end_game modundan play_again'e geçilebilir.");
        true
    }
}
