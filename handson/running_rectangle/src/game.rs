use rand::Rng;
use crate::constants::{MAX_SCREEN_HEIGHT, MAX_SCREEN_WIDTH};
use crate::{Position, Rectangle, Velocity};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::utility::Utility;

#[wasm_bindgen]
pub struct Game {
    rectangles: Vec<Rectangle>,
    pub max_width: u32,
    pub max_height: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let rectangles = vec![
            Rectangle::new(Position::new(rng.gen_range(0..32), 0), Utility::get_random_size(), Utility::get_random_color()),
            Rectangle::new(Position::new(rng.gen_range(100..136), 0), Utility::get_random_size(), Utility::get_random_color()),
            Rectangle::new(Position::new(rng.gen_range(200..236), 0), Utility::get_random_size(), Utility::get_random_color()),
            Rectangle::new(Position::new(rng.gen_range(300..336), 0), Utility::get_random_size(), Utility::get_random_color()),
            Rectangle::new(Position::new(rng.gen_range(400..436), 0), Utility::get_random_size(), Utility::get_random_color()),
            Rectangle::new(Position::new(rng.gen_range(500..536), 0), Utility::get_random_size(), Utility::get_random_color()),
        ];
        Self {
            max_width: MAX_SCREEN_WIDTH,
            max_height: MAX_SCREEN_HEIGHT,
            rectangles,
        }
    }

    pub fn add_rectangle(&mut self, rect: Rectangle) {
        self.rectangles.push(rect);
    }

    pub fn update(&mut self) {
        let velocity = Velocity::new(0, 1);
        for rect in &mut self.rectangles {
            rect.move_to(velocity.clone());
        }
    }

    pub fn get_rectangles_count(&self) -> usize {
        self.rectangles.len()
    }

    pub fn get_rectangles_at(&self, index: usize) -> Rectangle {
        self.rectangles[index].clone()
    }
}
