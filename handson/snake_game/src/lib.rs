use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    cell_count: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, start_index: usize) -> Self {
        Self {
            width: 8,
            cell_count: width * width,
            snake: Snake::new(start_index),
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
        // hücre indeksini buluyoruz
        let index = self.snake_head();
        // Hangi satırda olduğumuzu buluyoruz
        let row = index / self.width;
        // Hangi sütunda olduğumuzu buluyoruz
        let column = index % self.width;

        // 8 X 8 kareden oluştuğunu varsayarsak da saha dışına çıktığında 0dan başlaması için
        // modüle operatöründen yararlandık.

        // Yön tuşlarına göre yapılacak hareketlenmeleri match ifadesi ile karşılayabiliriz
        match self.snake.direction {
            Direction::Right => {
                let next_column = (index + 1) % self.width;
                self.snake.body[0].0 = (row * self.width) + next_column;
            }
            Direction::Left => {
                let next_column = (index - 1) % self.width;
                self.snake.body[0].0 = (row * self.width) + next_column;
            }
            Direction::Up => {
                let next_row = (row - 1) % self.width;
                self.snake.body[0].0 = (next_row * self.width) + column;
            }
            Direction::Down => {
                let next_row = (row + 1) % self.width;
                self.snake.body[0].0 = (next_row * self.width) + column;
            }
        }
    }
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(start_index: usize) -> Self {
        Self {
            body: vec![SnakeCell(start_index)],
            direction: Direction::Down,
        }
    }
}
