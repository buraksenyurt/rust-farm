/*
   Bu örnekte bir Oyun nesnesinin Init, Playing, PlayAgain, EndGame konumlarında olma halini ele aldığımız
   bir state pattern örneği söz konusu.

   Oyun nesnesi Init konumundan, Playing konumuna geçebilir ama EndGame konumuna geçemez.
   Playing konumundan EndGame konumuna geçebilir ama Init konumuna geçemez.
   EndGame konumundan PlayAgain ve Menu konumuna geçebilir.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_game_on_new_state_test() {
        let dragon_fly = Game::new(1);
        let current_state = dragon_fly.get_state();
        assert_eq!(current_state, "".to_string());
    }

    // Oyun nesnesi Init konumundan, Playing konumuna geçebilir ama EndGame konumuna geçemez.
    #[test]
    fn should_game_on_playing_state_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        assert_eq!(dragon_fly.state.get_state(), "PLAYING".to_string());
    }

    // Playing konumundan EndGame konumuna geçebilir.
    #[test]
    fn should_game_on_endgame_state_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        dragon_fly.end_game();
        assert_eq!(dragon_fly.state.get_state(), "END_GAME".to_string());
    }

    // Playing konumundan Init konumuna geçemez.
    #[test]
    fn game_should_not_be_init_state_when_playing_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        dragon_fly.init();
        assert_eq!(dragon_fly.state.get_state(), "PLAYING".to_string());
    }

    // EndGame konumundan PlayAgain konumuna geçebilir.
    #[test]
    fn should_game_on_play_again_state_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        dragon_fly.end_game();
        dragon_fly.play_again();
        assert_eq!(dragon_fly.state.get_state(), "PLAY_AGAIN".to_string());
    }

    // EndGame konumundan Menu konumuna geçebilir.
    #[test]
    fn should_game_on_menu_state_after_end_game_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        dragon_fly.end_game();
        dragon_fly.menu();
        assert_eq!(dragon_fly.state.get_state(), "MENU".to_string());
    }
}

struct Game {
    game_id: u32,
    state: Box<dyn GameState>,
}

impl Game {
    pub fn new(game_id: u32) -> Box<Game> {
        Box::new(Game {
            game_id,
            state: Box::new(InitState {
                name: "INIT".to_string(),
            }),
        })
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
        let result = self.state.init();
        if result {
            self.state = Box::new(PlayingState {
                name: "INIT".to_string(),
            })
        }
        result
    }
    fn playing(&mut self) -> bool {
        let result = self.state.playing();
        if result {
            self.state = Box::new(PlayingState {
                name: "PLAYING".to_string(),
            })
        }
        result
    }

    fn end_game(&mut self) -> bool {
        let result = self.state.end_game();
        if result {
            self.state = Box::new(EndGameState {
                name: "END_GAME".to_string(),
            })
        }
        result
    }

    fn menu(&mut self) -> bool {
        let result = self.state.menu();
        if result {
            self.state = Box::new(MenuState {
                name: "MENU".to_string(),
            })
        }
        result
    }

    fn play_again(&mut self) -> bool {
        let result = self.state.play_again();
        if result {
            self.state = Box::new(PlayAgainState {
                name: "PLAY_AGAIN".to_string(),
            })
        }
        result
    }
}

trait State {
    fn get_state(&self) -> String;
}
// Oyununun durumları arasındaki geçişlerin tanımlandığı trait
// Bu trait State trait'ini de devralmaktadır
trait GameState: State {
    fn init(&mut self) -> bool;
    fn playing(&mut self) -> bool;
    fn end_game(&mut self) -> bool;
    fn play_again(&mut self) -> bool;
    fn menu(&mut self) -> bool;
}

// Başlangıç durumunu temsil eden veri yapısı
struct InitState {
    name: String,
}

impl State for InitState {
    fn get_state(&self) -> String {
        self.name.clone()
    }
}

impl GameState for InitState {
    fn init(&mut self) -> bool {
        false
    }

    fn playing(&mut self) -> bool {
        true
    }

    fn end_game(&mut self) -> bool {
        false
    }

    fn menu(&mut self) -> bool {
        false
    }

    fn play_again(&mut self) -> bool {
        false
    }
}

// Menu durumunu temsil eden veri yapısı
struct MenuState {
    name: String,
}
impl State for MenuState {
    fn get_state(&self) -> String {
        self.name.clone()
    }
}

impl GameState for MenuState {
    fn init(&mut self) -> bool {
        true
    }

    fn playing(&mut self) -> bool {
        false
    }

    fn end_game(&mut self) -> bool {
        false
    }

    fn menu(&mut self) -> bool {
        false
    }

    fn play_again(&mut self) -> bool {
        false
    }
}

// Playing durumunu temsil eden veri yapısı
struct PlayingState {
    name: String,
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
        // Oyuncu yandığında end_game moduna geçilebilir
        true
    }

    fn menu(&mut self) -> bool {
        // esasında true olabilir belki. Oyunu oynarken Esc yapıp menu adımına geçmek de mümkündür.
        false
    }

    fn play_again(&mut self) -> bool {
        // Zaten oyun oynanıyor modda o nedenle play_again moduna da geçilmez.
        false
    }
}

// EndGame durumunu temsil eden veri yapısı
struct EndGameState {
    name: String,
}

impl State for EndGameState {
    fn get_state(&self) -> String {
        self.name.clone()
    }
}

// Oyun kaybedildiğinde geçilen EndGame durumudur.
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

struct PlayAgainState {
    name: String,
}

impl State for PlayAgainState {
    fn get_state(&self) -> String {
        self.name.clone()
    }
}

// Oyun kaybedildiğinde geçilen EndGame durumudur.
impl GameState for PlayAgainState {
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
        false
    }

    fn play_again(&mut self) -> bool {
        // Oyunun sonunda oyuncu yeniden oynamaya devam edebilir
        true
    }
}
