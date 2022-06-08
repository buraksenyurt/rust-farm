use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
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
            snake: Snake::new(start_index, 3),
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    // JS tarafından çağırılır ve yılanın basılan tuşa göre yön değiştirmesinde kullanılır
    pub fn change_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    // Yılanın gövde uzunluğunu döndürür
    pub fn get_snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // yılanın gövdesine ait hücreleri elde etmek için JS tarafından kullanılır
    // body'nin (vector türünün) bellek başlangıç adresini taşıyan bir raw pointer döndürülmektedir.
    // Bu sebeple borrowing kuralları işletilmez
    // Bu bilgiyi javascript tarafında alırken WASM nesnesinin memory özelliği kullanılır
    pub fn get_snake_body(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
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

pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(start_index: usize, body_size: usize) -> Self {
        let mut body = vec![];

        for i in 0..body_size {
            body.push(SnakeCell(start_index - i));
        }

        Self {
            body: body,
            direction: Direction::Right,
        }
    }
}
