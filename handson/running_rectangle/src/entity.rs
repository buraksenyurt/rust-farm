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
#[derive(Clone, Default)]
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
    pub fn set_x(&mut self, x: i32) {
        self.position.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.position.y = y;
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

    // AABB (Axis-Aligned Bounding Boxes)
    pub fn collided_with(&self, other: &Rectangle) -> bool {
        let self_right = self.position.x + self.size.width as i32;
        let self_bottom = self.position.y + self.size.height as i32;
        let other_right = other.position.x + other.size.width as i32;
        let other_bottom = other.position.y + other.size.height as i32;

        let x_overlap = self.position.x < other_right && self_right > other.position.x;
        let y_overlap = self.position.y < other_bottom && self_bottom > other.position.y;

        x_overlap && y_overlap
    }
}

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct Question {
    id: u32,
    text: String,
    answers: Vec<Answer>,
    level: Level,
}

#[wasm_bindgen]
#[derive(Default, Clone)]
pub enum Level {
    #[default]
    Easy,
    Medium,
    Hard,
}

impl Question {
    pub fn new(id: u32, text: String, level: Level) -> Self {
        Self {
            id,
            text,
            answers: vec![],
            level,
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

    pub fn get_correct_answer(&self) -> Answer {
        self.answers.iter().find(|a| a.is_correct).unwrap().clone()
    }

    pub fn get_level(&self) -> Level {
        self.level.clone()
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
