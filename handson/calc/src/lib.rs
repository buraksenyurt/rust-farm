extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[wasm_bindgen]
pub fn add(x: f64, y: f64) -> f64 {
    log("Toplama işlemi...", &format!("{}, {}", x, y));
    x + y
}

#[wasm_bindgen]
pub fn subtract(x: f64, y: f64) -> f64 {
    log("Çıkarma işlemi...", &format!("{}, {}", x, y));
    x - y
}

#[wasm_bindgen]
pub fn multiply(x: f64, y: f64) -> f64 {
    log("Çarpma işlemi...", &format!("{}, {}", x, y));
    x * y
}

#[wasm_bindgen]
pub fn divide(x: f64, y: f64) -> f64 {
    log("Bölme işlemi...", &format!("{}, {}", x, y));
    if y == 0.0 {
        panic!("Division by zero!");
    }
    x / y
}

#[cfg(not(target_arch = "wasm32"))]
fn log(message: &str, values: &str) {
    println!("{}, {}", message, values);
}

#[cfg(target_arch = "wasm32")]
fn log(message: &str, values: &str) {
    console::log_2(&message.into(), &values.into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_float_test() {
        let result = add(2.2, 2.2);
        assert_eq!(result, 4.4);
    }

    #[test]
    fn multiply_two_float_test() {
        let result = multiply(2.2, 2.0);
        assert_eq!(result, 4.4);
    }

    #[test]
    fn subtract_two_float_test() {
        let result = subtract(4.2,4.2);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn divide_two_float_test() {
        let result = divide(2.2, 2.0);
        assert_eq!(result, 1.1);
    }

    #[test]
    #[should_panic]
    fn divide_two_float_panics_test() {
        let result = divide(2.2, 0.0);
        assert_eq!(result, 1.1);
    }
}
