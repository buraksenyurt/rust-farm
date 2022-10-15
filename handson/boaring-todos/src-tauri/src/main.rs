#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::Rng;
use std::sync::{Arc, Mutex};
use tauri::{CustomMenuItem, Menu, State, Submenu};

#[derive(Default)]
struct BusinessValue(Arc<Mutex<i32>>);

#[tauri::command]
fn increase_value(v: i32, business_value: State<'_, BusinessValue>) -> String {
    let mut bv = business_value.0.lock().unwrap();
    *bv += v;
    format!("{bv}")
}

#[tauri::command]
fn decrease_value(v: i32, business_value: State<'_, BusinessValue>) -> String {
    let mut bv = business_value.0.lock().unwrap();
    *bv -= v;
    format!("{bv}")
}

#[tauri::command]
fn sum_of_two(x: String, y: String) -> String {
    if let Ok(x) = x.parse::<f32>() {
        if let Ok(y) = y.parse::<f32>() {
            let result = x + y;
            return format!("{} + {} = {}", x, y, result);
        }
    }
    "Toplama için sayısal değerler bekliyorum".to_string()
}

#[tauri::command]
async fn get_random_quote() -> String {
    let quotes=vec![
        "So many books, so little time. - Frank Zappa",
        "You only live once, but if you do it right, once is enough. - Mae West",
        "To live is the rarest thing in the world. Most people exist, that is all. - Oscar Wilde",
        "Without music, life would be a mistake. - Friedrich Nietzsche",
        "Remember, we're madly in love, so it's all right to kiss me anytime you feel like it. - Suzanne Collins",
        "Anyone who has never made a mistake has never tried anything new. - Albert Einstein",
        "The fear of death follows from the fear of life. A man who lives fully is prepared to die at any time. - Mark Twain",
        "Memories warm you up from the inside. But they also tear you apart. - Haruki Murakami",
        "I restore myself when I'm alone. - Marilyn Monroe"
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..quotes.len());
    quotes[index].to_string()
}

fn main() {
    tauri::Builder::default()
        .menu(
            Menu::new().add_submenu(Submenu::new(
                "Dosya",
                Menu::new()
                    .add_item(CustomMenuItem::new("open", "Aç").accelerator("cmdOrControl+O"))
                    .add_item(CustomMenuItem::new("save", "Kaydet").accelerator("cmdOrControl+S"))
                    .add_item(CustomMenuItem::new("close", "Kapat").accelerator("cmdOrControl+Q")),
            )),
        )
        .on_menu_event(|event| match event.menu_item_id() {
            "save" => {
                let _ = event.window().emit("menu-event", "File save").unwrap();
            }
            "open" => {
                let _ = event.window().emit("menu-event", "File open").unwrap();
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        .manage(BusinessValue(Default::default()))
        .invoke_handler(tauri::generate_handler![
            sum_of_two,
            get_random_quote,
            increase_value,
            decrease_value
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
