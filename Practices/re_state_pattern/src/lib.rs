/*
   Bu örnekte bir Oyun nesnesinin Menu, Playing ve EndGame konumlarında olma halini ele aldığımız
   bir state pattern örneği söz konusu.

   Oyun nesnesi Menu konumundan, Playing konumuna geçebilir ama EndGame konumuna geçemez.
   EndGame konumundan tekrar Playing'e (Replay seçeneği) veya Menu durumuna geçebilir.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_game_in_menu_state_at_the_beginning() {
        let dragon_fly = Game::new(1);
        let current_state = dragon_fly.get_state();
        assert_eq!(current_state, "".to_string());
    }
}

struct Game {
    game_id: u32,
    state: Box<dyn GameStateAction>,
}

impl State for Game {
    fn get_state(&self) -> String {
        "".to_string()
    }
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

trait State {
    fn get_state(&self) -> String;
}
// Oyununun durumları arasındaki geçişlerin tanımlandığı trait
// Bu trait State trait'ini de devralmaktadır
trait GameStateAction: State {
    fn playing(&mut self) -> bool;
    fn end_game(&mut self) -> bool;
    fn go_to_menu(&mut self) -> bool;
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
        true
    }

    fn end_game(&mut self) -> bool {
        false
    }

    fn go_to_menu(&mut self) -> bool {
        false
    }

    fn play_again(&mut self) -> bool {
        false
    }
}
