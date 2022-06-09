use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameGrid {
    pub size: GridSize,
}

#[wasm_bindgen]
#[derive(Clone,Copy)]
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
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            size: GridSize::new(rows, columns),
        }
    }
}
