use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameGrid {
    pub size: GridSize,
}

#[wasm_bindgen]
pub struct ColorCode(String);

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct GridSize {
    pub rows: usize,
    pub columns: usize,
}

#[wasm_bindgen]
pub fn get_random_color() -> String {
    let mut colors = vec![];

    colors.push(ColorCode("#861388".to_string()));
    colors.push(ColorCode("#E15A97".to_string()));
    colors.push(ColorCode("#EEABC4".to_string()));
    colors.push(ColorCode("#C799A6".to_string()));
    colors.push(ColorCode("#4B2840".to_string()));

    // colors[0] = ("DARK_MAGENTA".to_string(), "#861388".to_string());
    // colors[1] = ("FANDANGO_PINK".to_string(), "#E15A97".to_string());
    // colors[2] = ("NADESHIKO_PINK".to_string(), "#EEABC4".to_string());
    // colors[3] = ("TUSCANY".to_string(), "#C799A6".to_string());
    // colors[4] = ("DARK_BYZANTIUM".to_string(), "#4B2840".to_string());

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..5);

    colors[index].0.to_string()
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
