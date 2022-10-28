use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub album: String,
    pub artist: String,
}

impl Song {
    pub fn new(title: String, album: String, artist: String) -> Self {
        Self {
            title,
            album,
            artist,
        }
    }
}
