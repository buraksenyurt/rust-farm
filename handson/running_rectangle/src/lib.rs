extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    rectangles: Vec<Rectangle>,
    pub max_width: u32,
    pub max_height: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new(max_width: u32, max_height: u32) -> Self {
        let rectangles = vec![
            Rectangle::new(Position::new(0, 0), 32, 32, "#f5b041".to_string()),
            Rectangle::new(Position::new(128, 0), 16, 16, "#6c3483".to_string()),
            Rectangle::new(Position::new(64, 0), 24, 24, "#2874a6".to_string()),
            Rectangle::new(Position::new(256, 0), 8, 8, "#45b39d".to_string()),
        ];
        Self {
            max_width,
            max_height,
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
    width: u32,
    height: u32,
    color: String,
}

#[wasm_bindgen]
impl Rectangle {
    pub fn new(position: Position, width: u32, height: u32, color: String) -> Self {
        Self {
            position,
            width,
            height,
            color,
        }
    }

    pub fn move_to(&mut self, velocity: Velocity) {
        let new_x = self.position.x + velocity.x;
        if new_x >= 0 && (new_x as u32 + self.width <= 400) {
            self.position.x = new_x;
        }

        let new_y = self.position.y + velocity.y;
        if new_y >= 0 && (new_y as u32 + self.height <= 400) {
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
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
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
        let rect = Rectangle::new(position, 64, 64, "#5dade2".to_string());
        assert_eq!(rect.get_x(), 50);
        assert_eq!(rect.get_y(), 10);
        assert_eq!(rect.get_width(), 64);
        assert_eq!(rect.get_height(), 64);
    }

    #[test]
    fn move_left_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(-5, 0);
        let mut rect = Rectangle::new(position, 64, 64, "#5dade2".to_string());
        rect.move_to(velocity);
        assert_eq!(rect.get_x(), 45);
    }

    #[test]
    fn move_right_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(5, 0);
        let mut rect = Rectangle::new(position, 64, 64, "#5dade2".to_string());
        rect.move_to(velocity);
        assert_eq!(rect.get_x(), 55);
    }

    #[test]
    fn move_up_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(0, -5);
        let mut rect = Rectangle::new(position, 64, 64, "#5dade2".to_string());
        rect.move_to(velocity);
        assert_eq!(rect.get_y(), 5);
    }

    #[test]
    fn move_down_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(0, 5);
        let mut rect = Rectangle::new(position, 64, 64, "#5dade2".to_string());
        rect.move_to(velocity);
        assert_eq!(rect.get_y(), 15);
    }
}
