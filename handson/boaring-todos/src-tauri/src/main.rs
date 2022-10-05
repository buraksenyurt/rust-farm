#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sum_of_two])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
