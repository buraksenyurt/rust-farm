extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Rectangle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}