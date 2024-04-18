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
        Self {
            max_width,
            max_height,
            rectangles: Vec::new(),
        }
    }

    pub fn add_rectangle(&mut self, rect: Rectangle) {
        self.rectangles.push(rect);
    }

    pub fn update(&mut self) {
        for rect in &mut self.rectangles {
            rect.move_down();
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
pub struct Rectangle {
    position: Position,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Rectangle {
    pub fn new(position: Position, width: u32, height: u32) -> Self {
        Self {
            position,
            width,
            height,
        }
    }

    pub fn move_left(&mut self) {
        if (self.position.x as u32) > 0 {
            self.position.x -= 5;
        }
    }

    pub fn move_right(&mut self) {
        if (self.position.x as u32) < 400 - self.width - 5 {
            self.position.x += 5;
        }
    }

    pub fn move_up(&mut self) {
        if (self.position.y as u32) > 0 {
            self.position.y -= 5;
        }
    }

    pub fn move_down(&mut self) {
        if (self.position.y as u32) < 400 - self.height - 5 {
            self.position.y += 5;
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_rect_test() {
        let position = Position::new(50, 10);
        let rect = Rectangle::new(position, 64, 64);
        assert_eq!(rect.get_x(), 50);
        assert_eq!(rect.get_y(), 10);
        assert_eq!(rect.get_width(), 64);
        assert_eq!(rect.get_height(), 64);
    }

    #[test]
    fn move_left_rect_test() {
        let position = Position::new(50, 10);
        let mut rect = Rectangle::new(position, 64, 64);
        rect.move_left();
        assert_eq!(rect.get_x(), 45);
    }

    #[test]
    fn move_right_rect_test() {
        let position = Position::new(50, 10);
        let mut rect = Rectangle::new(position, 64, 64);
        rect.move_right();
        assert_eq!(rect.get_x(), 55);
    }

    #[test]
    fn move_up_rect_test() {
        let position = Position::new(50, 10);
        let mut rect = Rectangle::new(position, 64, 64);
        rect.move_up();
        assert_eq!(rect.get_y(), 5);
    }

    #[test]
    fn move_down_rect_test() {
        let position = Position::new(50, 10);
        let mut rect = Rectangle::new(position, 64, 64);
        rect.move_down();
        assert_eq!(rect.get_y(), 15);
    }
}
