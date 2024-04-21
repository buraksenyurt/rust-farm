mod colors;
mod constants;

extern crate wasm_bindgen;

use crate::colors::get_random_color;
use crate::constants::*;
use rand::prelude::SliceRandom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    rectangles: Vec<Rectangle>,
    pub max_width: u32,
    pub max_height: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        let rectangles = vec![
            Rectangle::new(Position::new(0, 0), get_random_size(), get_random_color()),
            Rectangle::new(Position::new(64, 0), get_random_size(), get_random_color()),
            Rectangle::new(Position::new(128, 0), get_random_size(), get_random_color()),
            Rectangle::new(Position::new(256, 0), get_random_size(), get_random_color()),
            Rectangle::new(Position::new(320, 0), get_random_size(), get_random_color()),
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

#[wasm_bindgen]
#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
impl Velocity {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Rectangle {
    position: Position,
    size: Size,
    color: String,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Size {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

enum BlockSize {
    Short,
    Tall,
    Grande,
    Venti,
}

impl BlockSize {
    pub fn to_size(&self) -> Size {
        match *self {
            BlockSize::Short => Size::new(8, 8),
            BlockSize::Tall => Size::new(16, 16),
            BlockSize::Grande => Size::new(32, 32),
            BlockSize::Venti => Size::new(64, 64),
        }
    }
}

pub fn get_random_size() -> Size {
    let mut rng = rand::thread_rng();
    let block_sizes = [
        BlockSize::Short,
        BlockSize::Tall,
        BlockSize::Grande,
        BlockSize::Venti,
    ];
    block_sizes.choose(&mut rng).unwrap().to_size()
}

#[wasm_bindgen]
impl Rectangle {
    pub fn new(position: Position, size: Size, color: String) -> Self {
        Self {
            position,
            size,
            color,
        }
    }

    pub fn move_to(&mut self, velocity: Velocity) {
        let new_x = self.position.x + velocity.x;
        if new_x >= 0 && (new_x as u32 + self.size.width <= MAX_SCREEN_WIDTH) {
            self.position.x = new_x;
        }

        let new_y = self.position.y + velocity.y;
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_rect_test() {
        let position = Position::new(50, 10);
        let rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string());
        assert_eq!(rect.get_x(), 50);
        assert_eq!(rect.get_y(), 10);
        assert_eq!(rect.get_width(), 64);
        assert_eq!(rect.get_height(), 64);
    }

    #[test]
    fn move_left_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(-5, 0);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string());
        rect.move_to(velocity);
        assert_eq!(rect.get_x(), 45);
    }

    #[test]
    fn move_right_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(5, 0);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string());
        rect.move_to(velocity);
        assert_eq!(rect.get_x(), 55);
    }

    #[test]
    fn move_up_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(0, -5);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string());
        rect.move_to(velocity);
        assert_eq!(rect.get_y(), 5);
    }

    #[test]
    fn move_down_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(0, 5);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string());
        rect.move_to(velocity);
        assert_eq!(rect.get_y(), 15);
    }
}
