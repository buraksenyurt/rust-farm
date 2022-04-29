use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
