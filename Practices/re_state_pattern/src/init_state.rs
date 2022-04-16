use crate::game_state::GameState;
use crate::state::State;

// Başlangıç durumunu temsil eden veri yapısı
pub struct InitState {
    pub name: String,
}

impl State for InitState {
    fn get_state(&self) -> String {
        self.name.clone()
    }
}

// Init halindeyken hangi diğer durumlara geçiş yapılabiliyorsa o davranışlar true değeri döner.
// Örneğin init modundan playing durumuna geçilebilir. Bu nedenle playing true dönmektedir.
impl GameState for InitState {
    fn init(&mut self) -> bool {
        println!("init modundan init'e geçilemez.");
        false
    }

    fn playing(&mut self) -> bool {
        println!("init modundan playing'e geçilebilir.");
        true
    }

    fn end_game(&mut self) -> bool {
        println!("init modundan end_game'e geçilemez.");
        false
    }

    fn menu(&mut self) -> bool {
        println!("init modundan menu'ye geçilemez.");
        false
    }

    fn play_again(&mut self) -> bool {
        println!("init modundan play_again'e geçilemez.");
        false
    }
}
