use tauri::State;

use crate::db;
use crate::models::{Note, NoteInput};
use crate::AppState;

#[tauri::command]
pub fn get_random_note(state: State<'_, AppState>) -> Result<Option<Note>, String> {
    let conn = state.db.lock().map_err(|_| "Veritabanı durumu kilitli".to_string())?;
    db::get_random_note(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_notes(state: State<'_, AppState>) -> Result<Vec<Note>, String> {
    let conn = state.db.lock().map_err(|_| "Veritabanı durumu kilitli".to_string())?;
    db::list_notes(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_notes_sorted(
    state: State<'_, AppState>,
    column: String,
    order: String,
) -> Result<Vec<Note>, String> {
    let conn = state.db.lock().map_err(|_| "Veritabanı durumu kilitli".to_string())?;
    db::list_notes_sorted(&conn, &column, &order).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_note(state: State<'_, AppState>, id: i64) -> Result<Option<Note>, String> {
    let conn = state.db.lock().map_err(|_| "Veritabanı durumu kilitli".to_string())?;
    db::get_note(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_note(state: State<'_, AppState>, input: NoteInput) -> Result<Note, String> {
    let conn = state.db.lock().map_err(|_| "Veritabanı durumu kilitli".to_string())?;
    db::add_note(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn archive_note(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|_| "Veritabanı durumu kilitli".to_string())?;
    db::archive_note(&conn, id).map_err(|e| e.to_string())
}
