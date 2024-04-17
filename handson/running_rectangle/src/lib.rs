extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Rectangle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Rectangle {
    pub fn new() -> Self {
        Self {
            x: 50,
            y: 10,
            width: 64,
            height: 64,
        }
    }

    pub fn move_left(&mut self) {
        self.x -= 5;
    }

    pub fn move_right(&mut self) {
        self.x += 5;
    }

    pub fn move_up(&mut self) {
        self.y -= 5;
    }

    pub fn move_down(&mut self) {
        self.y += 5;
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
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
        let rect = Rectangle::new();
        assert_eq!(rect.get_x(), 50);
        assert_eq!(rect.get_y(), 10);
        assert_eq!(rect.get_width(), 64);
        assert_eq!(rect.get_height(), 64);
    }

    #[test]
    fn move_left_rect_test() {
        let mut rect = Rectangle::new();
        rect.move_left();
        assert_eq!(rect.get_x(), 45);
    }

    #[test]
    fn move_right_rect_test() {
        let mut rect = Rectangle::new();
        rect.move_right();
        assert_eq!(rect.get_x(), 55);
    }

    #[test]
    fn move_up_rect_test() {
        let mut rect = Rectangle::new();
        rect.move_up();
        assert_eq!(rect.get_y(), 5);
    }

    #[test]
    fn move_down_rect_test() {
        let mut rect = Rectangle::new();
        rect.move_down();
        assert_eq!(rect.get_y(), 15);
    }
}
