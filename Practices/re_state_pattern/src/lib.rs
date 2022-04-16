/*
   Bu örnekte bir Oyun nesnesinin Menu, Playing ve EndGame konumlarında olma halini ele aldığımız
   bir state pattern örneği söz konusu.

   Oyun nesnesi Menu konumundan, Playing konumuna geçebilir ama EndGame konumuna geçemez.
   Playing konumundan EndGame konumuna geçebilir ama Menu konumuna geçemez.
   EndGame konumundan tekrar Playing'e (Replay seçeneği) veya Menu durumuna geçebilir.
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

    #[test]
    fn should_game_on_playing_state_test() {
        let mut dragon_fly = Game::new(1);
        let current_state = dragon_fly.get_state();
        assert_eq!(current_state, "".to_string());
        let actual = dragon_fly.playing();
        assert_eq!(actual, true);
        assert_eq!(dragon_fly.state.get_state(), "PLAYING".to_string());
    }

    #[test]
    fn should_game_on_endgame_state_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        dragon_fly.end_game();
        assert_eq!(dragon_fly.state.get_state(), "END_GAME".to_string());
    }

    #[test]
    fn should_game_on_play_again_state_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        dragon_fly.end_game();
        dragon_fly.play_again();
        assert_eq!(dragon_fly.state.get_state(), "PLAY_AGAIN".to_string());
    }

    #[test]
    fn should_game_on_menu_state_from_end_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        dragon_fly.end_game();
        dragon_fly.menu();
        assert_eq!(dragon_fly.state.get_state(), "MENU_FROM_END".to_string());
    }
}

struct Game {
    game_id: u32,
    state: Box<dyn GameStateAction>,
}

impl Game {
    pub fn new(game_id: u32) -> Box<Game> {
        Box::new(Game {
            game_id,
            state: Box::new(MenuState {
                name: "MENU".to_string(),
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
// Bu geçişleri kontrol altına almak için GameStateAction trait'i üstünden gelen
// davranışların Game veri yapısı için uygulaması yapılır.
impl GameStateAction for Game {
    fn playing(&mut self) -> bool {
        // Oyunun güncel durumuna bakılır.
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
        // Oyun nesnesninin güncel durumuna bakılır
        // Eğer end_game konumundaysak, menüye tekrar dönebiliriz.
        let result = self.state.end_game();
        if result {
            self.state = Box::new(MenuState {
                name: "MENU_FROM_END".to_string(),
            })
        }
        result
    }

    fn play_again(&mut self) -> bool {
        // Oyun nesnesninin güncel durumuna bakılır
        // Eğer end_game konumundaysak, yeniden oynamaya dönebiliriz.
        let result = self.state.end_game();
        if result {
            self.state = Box::new(PlayingState {
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
trait GameStateAction: State {
    fn playing(&mut self) -> bool;
    fn end_game(&mut self) -> bool;
    fn menu(&mut self) -> bool;
    fn play_again(&mut self) -> bool;
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

// Menu adımından playing durumuna geçilebilir.
// Bu sebepten sadece playing uygulanabilirdir.
// playing true dönerken diğer trait fonksiyonları false dönecektir.
impl GameStateAction for MenuState {
    fn playing(&mut self) -> bool {
        // playing moda geçebiliriz.
        true
    }

    fn end_game(&mut self) -> bool {
        // end_game moduna geçemeyiz
        false
    }

    fn menu(&mut self) -> bool {
        // zaten menüdeyiz
        false
    }

    fn play_again(&mut self) -> bool {
        // yeniden oyna moduna da geçemeyiz
        false
    }
}

// Menu durumunu temsil eden veri yapısı
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
impl GameStateAction for PlayingState {
    fn playing(&mut self) -> bool {
        // Zaten playing moddayız
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
impl GameStateAction for EndGameState {
    fn playing(&mut self) -> bool {
        false
    }

    fn end_game(&mut self) -> bool {
        // Zaten EndGame durumundayız
        true
    }

    fn menu(&mut self) -> bool {
        // Oyunun sonunda oyuncu menüye dönebilir
        true
    }

    fn play_again(&mut self) -> bool {
        // Oyunun sonunda oyuncu yeniden oynamaya devam edebilir
        true
    }
}
