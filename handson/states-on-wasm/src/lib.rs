use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    state: GameState,
}

#[derive(Debug)]
enum GameState {
    Menu,
    Playing,
    Credits,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::Menu,
        }
    }

    pub fn update_state(&mut self, new_state: &str) {
        self.state = match new_state {
            "menu" => GameState::Menu,
            "playing" => GameState::Playing,
            "credits" => GameState::Credits,
            _ => {
                return;
            }
        };
    }
}
