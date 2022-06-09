use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameGrid {
    pub size: GridSize,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct GridSize {
    pub rows: usize,
    pub columns: usize,
}

#[wasm_bindgen]
impl GridSize {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self { rows, columns }
    }
}

#[wasm_bindgen]
impl GameGrid {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let row_count = rng.gen_range(5..16);
        let column_count = rng.gen_range(5..16);
        Self {
            size: GridSize::new(row_count, column_count),
        }
    }

    pub fn get_max_len(&self) -> usize {
        if self.size.rows > self.size.columns {
            self.size.rows
        } else {
            self.size.columns
        }
    }
}
