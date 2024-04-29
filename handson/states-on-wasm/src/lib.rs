use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::HtmlElement;

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
    pub fn new() -> Self {
        let game = Game {
            state: GameState::Menu,
        };
        game.update_visibility();
        game
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
        self.update_visibility();
    }

    fn update_visibility(&self) {
        let document = window().unwrap().document().unwrap();
        let menu = document
            .get_element_by_id("divMenu")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        let game = document
            .get_element_by_id("divGame")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        let credits = document
            .get_element_by_id("divCredits")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        menu.style().set_property("display", "none").unwrap();
        game.style().set_property("display", "none").unwrap();
        credits.style().set_property("display", "none").unwrap();

        match self.state {
            GameState::Menu => {
                menu.style().set_property("display", "block").unwrap();
            }
            GameState::Playing => {
                game.style().set_property("display", "block").unwrap();
            }
            GameState::Credits => {
                credits.style().set_property("display", "block").unwrap();
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
