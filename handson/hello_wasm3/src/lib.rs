use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn print_temperature(city: &str) -> String {
    let mut rng = rand::thread_rng();
    let temperature = rng.gen_range(-10..50);
    format!("{} için sıcaklık {} derece", city, temperature)
}
