use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn print_temperature(city: &str) {
    println!("{} şehrindeki sıcaklık 32 derecedir", city);
}
