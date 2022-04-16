use crate::game_state::GameState;
use crate::state::State;

// Menu durumunu temsil eden veri yapısı
pub struct MenuState {
    pub name: String,
}

impl State for MenuState {
    fn get_state(&self) -> String {
        self.name.clone()
    }
}

// Menu durumundan init durumuna geçilebilir. Bu nedenle sadece o davranış true dönmektedir.
impl GameState for MenuState {
    fn init(&mut self) -> bool {
        println!("menu modundan init'e geçilebilir.");
        true
    }

    fn playing(&mut self) -> bool {
        println!("menu modundan playing'e geçilemez.");
        false
    }

    fn end_game(&mut self) -> bool {
        println!("menu modundan end_game'e geçilemez.");
        false
    }

    fn menu(&mut self) -> bool {
        println!("menu modundan menu'ye geçilemez.");
        false
    }

    fn play_again(&mut self) -> bool {
        println!("menu modundan play_again'e geçilemez.");
        false
    }
}
