use crate::end_game::EndGameState;
use crate::game_state::GameState;
use crate::init_state::InitState;
use crate::menu_state::MenuState;
use crate::play_again::PlayAgainState;
use crate::playing::PlayingState;
use crate::state::State;
use std::fmt::{Display, Formatter};

pub struct Game {
    game_id: u32,
    pub state: Box<dyn GameState>,
}

impl Game {
    pub fn new(game_id: u32) -> Box<Game> {
        println!("Yeni bir oyun nesnesi oluşturuldu. #{}", game_id);
        Box::new(Game {
            game_id,
            state: Box::new(InitState {
                name: "INIT".to_string(),
            }),
        })
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Oyun #{}. Durum => {}",
            self.game_id,
            self.state.get_state()
        )
    }
}

impl State for Game {
    fn get_state(&self) -> String {
        "".to_string()
    }
}

// Oyun nesnesi üstünde durumlar arası geçiş söz konusudur.
// Bu geçişleri kontrol altına almak için GameState trait'i üstünden gelen
// davranışların Game veri yapısı için uygulaması yapılır.
impl GameState for Game {
    fn init(&mut self) -> bool {
        println!("\tINIT çağrısı.");
        let result = self.state.init();
        if result {
            self.state = Box::new(PlayingState {
                name: "INIT".to_string(),
            })
        };
        result
    }
    fn playing(&mut self) -> bool {
        println!("\tPLAYING çağrısı.");
        let result = self.state.playing();
        if result {
            self.state = Box::new(PlayingState {
                name: "PLAYING".to_string(),
            })
        }
        result
    }

    fn end_game(&mut self) -> bool {
        println!("\tEND_GAME çağrısı.");
        let result = self.state.end_game();
        if result {
            self.state = Box::new(EndGameState {
                name: "END_GAME".to_string(),
            })
        }
        result
    }

    fn menu(&mut self) -> bool {
        println!("\tMENU çağrısı.");
        let result = self.state.menu();
        if result {
            self.state = Box::new(MenuState {
                name: "MENU".to_string(),
            })
        }
        result
    }

    fn play_again(&mut self) -> bool {
        println!("\tPLAY_AGAIN çağrısı.");
        let result = self.state.play_again();
        if result {
            self.state = Box::new(PlayAgainState {
                name: "PLAY_AGAIN".to_string(),
            })
        }
        result
    }
}
