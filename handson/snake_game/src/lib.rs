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
        // Bulunduğumuz satır ve sütun bilgilerini hesaplıyoruz
        let (row, column) = self.index_to_cell(index);

        // 8 X 8 kareden oluştuğunu varsayarsak da saha dışına çıktığında 0dan başlaması için
        // modüle operatöründen yararlandık.
        // Yön tuşlarına göre yapılacak hareketlenmeleri match ifadesi ile karşılayabiliriz
        let (row, column) = match self.snake.direction {
            Direction::Right => (row, (column + 1) % self.width),
            Direction::Left => (row, (column - 1) % self.width),
            Direction::Up => ((row - 1) % self.width, column),
            Direction::Down => ((row + 1) % self.width, column),
        };
        let next_index = self.cell_to_index(row, column);
        self.move_to_next(next_index);
    }

    fn move_to_next(&mut self, index: usize) {
        self.snake.body[0].0 = index;
    }

    fn cell_to_index(&self, row: usize, column: usize) -> usize {
        (row * self.width) + column
    }

    fn index_to_cell(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
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
            direction: Direction::Up,
        }
    }
}
