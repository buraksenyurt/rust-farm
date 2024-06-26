use crate::constants::{
    LANE_COUNT, MAX_SCREEN_HEIGHT, MAX_SCREEN_WIDTH, MAX_VERTICAL_SPEED, MIN_VERTICAL_SPEED,
};
use crate::entity::*;
use crate::instrument::{Position, Size, Velocity};
use crate::lane_manager::{Column, LaneManager};
use crate::question_manager::QuestionManager;
use crate::utility::Utility;
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::{HtmlElement, SvgElement};

#[derive(Debug)]
enum GameState {
    Menu,
    Playing,
    End,
}

#[wasm_bindgen]
pub struct Game {
    rectangles: Vec<Rectangle>,
    player: Rectangle,
    pub max_width: u32,
    pub max_height: u32,
    question: Question,
    state: GameState,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        let game = Self {
            max_width: MAX_SCREEN_WIDTH,
            max_height: MAX_SCREEN_HEIGHT,
            rectangles: vec![],
            question: Question::default(),
            player: Rectangle::default(),
            state: GameState::Menu,
        };
        game.update_visibility();
        game
    }

    pub fn init(&mut self) {
        self.rectangles.clear();

        let mut rng = rand::thread_rng();
        let lane_manager = LaneManager::new();
        let question_manager = QuestionManager::init();
        let question = question_manager
            .get_question(rng.gen_range(1..=question_manager.get_question_count()))
            .unwrap()
            .clone();
        let mut q_index: Vec<u32> = (0..=LANE_COUNT as u32).collect();
        q_index.shuffle(&mut rng);
        for (lane_index, &i) in q_index.iter().enumerate() {
            let rectangle = Rectangle::new(
                Position::new(
                    rng.gen_range(lane_manager.get_lane_range(Column::from(lane_index as u32))),
                    0,
                ),
                Utility::get_random_size(),
                Utility::get_random_color(),
                Self::get_random_velocity(&mut rng),
                question.get_answer_at(i).get_text(),
            );
            self.rectangles.push(rectangle);
        }

        let player = Rectangle::new(
            Position::new(0, MAX_SCREEN_HEIGHT as i32 - 64),
            Size::new(64, 64),
            "Black".to_string(),
            Velocity::new(0, 0),
            "".to_string(),
        );

        self.question = question;
        self.player = player;
        self.setup_scene();
    }

    fn setup_scene(&self) {
        let document = window().unwrap().document().unwrap();
        if let Ok(schene_element) = document
            .get_element_by_id("scene")
            .unwrap()
            .dyn_into::<SvgElement>()
        {
            schene_element
                .set_attribute("width", self.max_width.to_string().as_str())
                .unwrap();
            schene_element
                .set_attribute("height", self.max_height.to_string().as_str())
                .unwrap();
        }
    }

    fn get_random_velocity(rng: &mut ThreadRng) -> Velocity {
        let y = rng.gen_range(MIN_VERTICAL_SPEED..MAX_VERTICAL_SPEED);
        Velocity::new(0, y)
    }

    pub fn add_rectangle(&mut self, rect: Rectangle) {
        self.rectangles.push(rect);
    }

    pub fn get_player(&self) -> Rectangle {
        self.player.clone()
    }

    pub fn update_state(&mut self, new_state: &str) {
        self.state = match new_state {
            "menu" => GameState::Menu,
            "playing" => GameState::Playing,
            "endGame" => GameState::End,
            _ => return,
        };
        self.update_visibility();
    }
    pub fn update(&mut self) {
        match self.state {
            GameState::Menu => {}
            GameState::Playing => {
                for rect in &mut self.rectangles {
                    rect.move_to();
                }
            }
            GameState::End => {}
        }
    }
    pub fn draw(&mut self) {
        if let Some(document) = window().unwrap().document() {
            if let Some(container) = document.get_element_by_id("blocksContainer") {
                if let Ok(container_element) = container.clone().dyn_into::<SvgElement>() {
                    container_element.set_inner_html("");
                    for rect in self.rectangles.iter() {
                        let x = rect.get_x() as f32;
                        let y = rect.get_y() as f32;
                        let width = rect.get_width() as f32;
                        let height = rect.get_height() as f32;
                        let answer = rect.get_answer_text().clone();

                        let block = document
                            .create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")
                            .unwrap();
                        block
                            .set_attribute(
                                "style",
                                &format!(
                                    "x: {}px; y: {}px; width: {}px; height: {}px;",
                                    x, y, width, height
                                ),
                            )
                            .unwrap();
                        block
                            .set_attribute("fill", rect.clone().get_color().as_str())
                            .unwrap();
                        container.append_child(&block).unwrap();

                        let text = document
                            .create_element_ns(Some("http://www.w3.org/2000/svg"), "text")
                            .unwrap();
                        text.set_attribute("x", (x + width / 2.0).to_string().as_str())
                            .unwrap();
                        text.set_attribute("y", (y + height / 2.0 + 5.0).to_string().as_str())
                            .unwrap();
                        text.set_attribute("text-anchor", "middle").unwrap();
                        text.set_attribute("fill", "white").unwrap();
                        text.set_attribute("font-size", "18px").unwrap();
                        text.set_attribute("font-weight", "bold").unwrap();
                        text.set_text_content(Some(&answer));
                        container.append_child(&text).unwrap();
                    }
                }
            }
        }
        self.draw_hud();
        self.draw_player();
    }

    pub fn update_player(&mut self, position: Position) {
        if let GameState::Playing = self.state {
            self.player.set_x(position.x);
            self.player.set_y(position.y);
        }
    }

    fn draw_player(&self) {
        let document = window().unwrap().document().unwrap();
        if let Ok(player_element) = document
            .get_element_by_id("player")
            .unwrap()
            .dyn_into::<SvgElement>()
        {
            player_element
                .set_attribute(
                    "style",
                    &format!(
                        "width:{}px; height:{}px; x:{}px; y:{}px;",
                        self.player.get_width(),
                        self.player.get_height(),
                        self.player.get_x(),
                        self.player.get_y()
                    ),
                )
                .unwrap();
        }
    }

    fn draw_hud(&self) {
        let document = window().unwrap().document().unwrap();
        if let Ok(hud_element) = document
            .get_element_by_id("hud")
            .unwrap()
            .dyn_into::<HtmlElement>()
        {
            let (bg_color, font_color) = match self.question.get_level() {
                Level::Easy => ("#00A36C", "#FFFFFF"),
                Level::Medium => ("#4169E1", "#F2D2BD"),
                Level::Hard => ("#8B0000", "#F2D2BD"),
            };
            hud_element
                .set_attribute(
                    "style",
                    &format!(
                        "background-color: {};color:{}; width:{}, height: 64px",
                        bg_color, font_color, self.max_width
                    ),
                )
                .unwrap();
        }

        if let Ok(question_element) = document
            .get_element_by_id("questionText")
            .unwrap()
            .dyn_into::<HtmlElement>()
        {
            question_element.set_inner_text(self.question.get_text().as_str());
        }
    }

    pub fn get_current_question_text(&self) -> String {
        self.question.get_text()
    }

    pub fn collision_check(&self) -> bool {
        for rect in self.rectangles.iter() {
            if self.player.collided_with(rect)
                && rect.get_answer_text() == self.question.get_correct_answer().get_text()
            {
                return true;
            }
        }
        false
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
        let end_game = document
            .get_element_by_id("divEndGame")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        menu.style().set_property("display", "none").unwrap();
        game.style().set_property("display", "none").unwrap();
        end_game.style().set_property("display", "none").unwrap();

        match self.state {
            GameState::Menu => {
                menu.style().set_property("display", "block").unwrap();
            }
            GameState::Playing => {
                game.style().set_property("display", "block").unwrap();
            }
            GameState::End => {
                end_game.style().set_property("display", "block").unwrap();
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
