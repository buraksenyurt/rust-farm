mod commands;
mod db;
mod models;

use std::fs;
use std::sync::Mutex;

use rusqlite::Connection;
use tauri::Manager;

use commands::{add_note, archive_note, get_note, get_random_note, list_notes, list_notes_sorted};

/// SQLite bağlantısı, komutlar Tauri'nin worker thread pool'unda çalıştığından
/// Mutex ile korunuyor (sys-trace'teki Mutex<System> deseniyle aynı gerekçe).
pub struct AppState {
    pub db: Mutex<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            fs::create_dir_all(&data_dir)?;
            let conn = Connection::open(data_dir.join("notes.db"))?;
            db::init_db(&conn)?;
            app.manage(AppState { db: Mutex::new(conn) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_random_note,
            list_notes,
            list_notes_sorted,
            get_note,
            add_note,
            archive_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
