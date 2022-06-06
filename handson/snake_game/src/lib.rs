use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub struct World {
    width: usize,
    cell_count: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> Self {
        let width = 8;
        Self {
            width: 8,
            cell_count: width * width,
            snake: Snake::new(18),
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    // Yılanın pozisyonunu değiştiren fonksiyon
    pub fn update_position(&mut self) {
        let index = self.snake_head();
        // 0ncı elemanın index değerini 1 artırıyor.
        // 8 X 8 kareden oluştuğunu varsayarsak da saha dışına çıktığında 0dan başlaması için
        // modüle operatöründen yararlandık.
        self.snake.body[0].0 = (index + 1) % self.cell_count;
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
