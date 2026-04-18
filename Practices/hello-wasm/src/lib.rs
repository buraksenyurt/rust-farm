use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum(x: f32, y: f32) -> f32 {
    x + y
}

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }
//
// #[wasm_bindgen]
// pub fn greet(user_name: &str) {
//     alert(&format!("Merhaba {}", user_name));
// }
