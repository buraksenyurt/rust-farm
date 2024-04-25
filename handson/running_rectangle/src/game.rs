use crate::constants::{
    MAX_SCREEN_HEIGHT, MAX_SCREEN_WIDTH, MAX_VERTICAL_SPEED, MIN_VERTICAL_SPEED,
};
use crate::lane_manager::{Column, LaneManager};
use crate::utility::Utility;
use rand::rngs::ThreadRng;
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::entity::*;
use crate::instrument::{Position, Velocity};

#[wasm_bindgen]
pub struct Game {
    rectangles: Vec<Rectangle>,
    pub max_width: u32,
    pub max_height: u32,
}

#[wasm_bindgen]
impl Game {
    fn get_random_velocity(rng: &mut ThreadRng) -> Velocity {
        let y = rng.gen_range(MIN_VERTICAL_SPEED..MAX_VERTICAL_SPEED);
        Velocity::new(0, y)
    }
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let lane_manager = LaneManager::new();

        let rectangles = vec![
            Rectangle::new(
                Position::new(rng.gen_range(lane_manager.get_lane_range(Column::Zero)), 0),
                Utility::get_random_size(),
                Utility::get_random_color(),
                Self::get_random_velocity(&mut rng),
            ),
            Rectangle::new(
                Position::new(rng.gen_range(lane_manager.get_lane_range(Column::One)), 0),
                Utility::get_random_size(),
                Utility::get_random_color(),
                Self::get_random_velocity(&mut rng),
            ),
            Rectangle::new(
                Position::new(rng.gen_range(lane_manager.get_lane_range(Column::Two)), 0),
                Utility::get_random_size(),
                Utility::get_random_color(),
                Self::get_random_velocity(&mut rng),
            ),
            Rectangle::new(
                Position::new(rng.gen_range(lane_manager.get_lane_range(Column::Three)), 0),
                Utility::get_random_size(),
                Utility::get_random_color(),
                Self::get_random_velocity(&mut rng),
            ),
            Rectangle::new(
                Position::new(rng.gen_range(lane_manager.get_lane_range(Column::Four)), 0),
                Utility::get_random_size(),
                Utility::get_random_color(),
                Self::get_random_velocity(&mut rng),
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
        for rect in &mut self.rectangles {
            rect.move_to();
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
