use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Song {
    pub title: String,
    pub album: String,
    pub artist: String,
}
