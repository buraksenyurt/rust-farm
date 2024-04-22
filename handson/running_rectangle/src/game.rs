use crate::constants::{MAX_SCREEN_HEIGHT, MAX_SCREEN_WIDTH};
use crate::lane_manager::{Column, LaneManager};
use crate::utility::Utility;
use crate::{Position, Rectangle, Velocity};
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

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
        let lane_manager = LaneManager::new();

        let rectangles = vec![
            Rectangle::new(
                Position::new(rng.gen_range(lane_manager.get_lane_range(Column::Zero)), 0),
                Utility::get_random_size(),
                Utility::get_random_color(),
            ),
            Rectangle::new(
                Position::new(rng.gen_range(lane_manager.get_lane_range(Column::One)), 0),
                Utility::get_random_size(),
                Utility::get_random_color(),
            ),
            Rectangle::new(
                Position::new(rng.gen_range(lane_manager.get_lane_range(Column::Two)), 0),
                Utility::get_random_size(),
                Utility::get_random_color(),
            ),
            Rectangle::new(
                Position::new(rng.gen_range(lane_manager.get_lane_range(Column::Three)), 0),
                Utility::get_random_size(),
                Utility::get_random_color(),
            ),
            Rectangle::new(
                Position::new(rng.gen_range(lane_manager.get_lane_range(Column::Four)), 0),
                Utility::get_random_size(),
                Utility::get_random_color(),
            ),
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

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
