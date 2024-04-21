mod colors;
mod constants;
mod game;
mod test;
mod utility;

use crate::constants::*;
use wasm_bindgen::prelude::*;

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
            BlockSize::Short => Size::new(24, 24),
            BlockSize::Tall => Size::new(32, 32),
            BlockSize::Grande => Size::new(48, 48),
            BlockSize::Venti => Size::new(64, 64),
        }
    }
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
