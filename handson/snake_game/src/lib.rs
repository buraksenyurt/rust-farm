use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> Self {
        Self {
            width: 8,
            snake: Snake::new(18),
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
    }
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    fn new(start_index: usize) -> Self {
        Self {
            body: vec![SnakeCell(start_index)],
        }
    }
}
