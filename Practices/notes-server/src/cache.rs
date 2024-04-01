use crate::entity::Note;
use crate::utility::get_file_path;
use once_cell::sync::Lazy;
use serde_json::from_str;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::fs::{metadata, File};
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct NotesCache {
    pub notes: Vec<Note>,
    pub last_modified: SystemTime,
}

pub static CACHED_NOTES: Lazy<Arc<Mutex<Option<NotesCache>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub async fn update_cache_if_needed() -> Arc<Mutex<Option<NotesCache>>> {
    let path = get_file_path("notes.json");
    let metadata = metadata(path.clone()).await.unwrap();
    let last_modified = metadata.modified().unwrap();
    let mut cache = CACHED_NOTES.lock().await;

    match *cache {
        Some(ref cache) if cache.last_modified >= last_modified => Arc::clone(&CACHED_NOTES),
        _ => {
            println!("Reading to cache");
            let mut file = File::open(path).await.unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).await.unwrap();

            let notes: Vec<Note> = from_str(&contents).unwrap();
            *cache = Some(NotesCache {
                notes,
                last_modified,
            });
            Arc::clone(&CACHED_NOTES)
        }
    }
}
