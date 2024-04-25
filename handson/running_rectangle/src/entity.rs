use crate::constants::{MAX_SCREEN_HEIGHT, MAX_SCREEN_WIDTH};
use crate::instrument::{Position, Size, Velocity};
use wasm_bindgen::prelude::wasm_bindgen;

pub enum BlockSize {
    Short,
    Tall,
    Grande,
    Venti,
}
impl BlockSize {
    pub fn to_size(&self) -> Size {
        match *self {
            BlockSize::Short => Size::new(24, 24),
            BlockSize::Tall => Size::new(32, 32),
            BlockSize::Grande => Size::new(48, 48),
            BlockSize::Venti => Size::new(64, 64),
        }
    }
}
#[wasm_bindgen]
#[derive(Clone)]
pub struct Rectangle {
    position: Position,
    size: Size,
    color: String,
    velocity: Velocity,
    answer_text: String,
}
#[wasm_bindgen]
impl Rectangle {
    pub fn new(
        position: Position,
        size: Size,
        color: String,
        velocity: Velocity,
        answer_text: String,
    ) -> Self {
        Self {
            position,
            size,
            color,
            velocity,
            answer_text,
        }
    }

    pub fn change_velocity(&mut self, x: i32, y: i32) {
        self.velocity = Velocity::new(x, y)
    }

    pub fn move_to(&mut self) {
        let new_x = self.position.x + self.velocity.x;
        if new_x >= 0 && (new_x as u32 + self.size.width <= MAX_SCREEN_WIDTH) {
            self.position.x = new_x;
        }

        let new_y = self.position.y + self.velocity.y;
        if new_y >= 0 && (new_y as u32 + self.size.height <= MAX_SCREEN_HEIGHT) {
            self.position.y = new_y;
        }
    }

    pub fn get_x(&self) -> i32 {
        self.position.x
    }
    pub fn get_y(&self) -> i32 {
        self.position.y
    }
    pub fn get_width(&self) -> u32 {
        self.size.width
    }
    pub fn get_height(&self) -> u32 {
        self.size.height
    }
    pub fn get_color(self) -> String {
        self.color
    }
    pub fn get_answer_text(&self) -> String {
        String::from(&self.answer_text)
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Question {
    id: u32,
    text: String,
    answers: Vec<Answer>,
}

impl Question {
    pub fn new(id: u32, text: String) -> Self {
        Self {
            id,
            text,
            answers: vec![],
        }
    }

    pub fn add_answer(&mut self, answer: Answer) {
        self.answers.push(answer);
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_text(&self) -> String {
        String::from(&self.text)
    }

    pub fn get_answers(&self) -> Vec<Answer> {
        self.answers.clone()
    }

    pub fn get_answer_at(&self, id: u32) -> Answer {
        self.answers.iter().find(|a| a.id == id).unwrap().clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Answer {
    id: u32,
    text: String,
    is_correct: bool,
}

impl Answer {
    pub fn new(id: u32, text: String, is_correct: bool) -> Self {
        Self {
            id,
            text,
            is_correct,
        }
    }

    pub fn is_correct(&self) -> bool {
        self.is_correct
    }

    pub fn get_text(self) -> String {
        self.text
    }
}
