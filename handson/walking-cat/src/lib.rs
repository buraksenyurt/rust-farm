use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // // This provides better error messages in debug mode.
    // // It's disabled in release mode so it doesn't bloat up the file size.
    // #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // DOM'a inip mainCanvas elementini alıyoruz.
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("mainCanvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    // Canvas elementi içerisinde çizim yapmak için context nesnesini yakalıyoruz
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    console::log_1(&JsValue::from_str(
        "Rust ve WASM ile Web tabanlı oyun geliştirme macerası.",
    ));

    context.move_to(300.0, 0.0);
    context.begin_path();
    context.line_to(0.0, 300.0);
    context.line_to(300.0, 300.0);
    context.line_to(300.0, 0.0);
    context.close_path();
    context.stroke();
    context.fill();

    Ok(())
}
