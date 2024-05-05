use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: usize,
    pub title: String,
    pub body: String,
    pub publisher: String,
    pub author: String,
    #[serde(rename = "mediaType")]
    pub media_type: MediaType,
    pub year: usize,
    pub month: String,
    pub day: usize,
    #[serde(default)]
    pub externals: Option<Vec<External>>,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct External {
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum MediaType {
    Gazete,
    Dergi,
    Dijital,
    Kitap,
    Podcast,
    Medium,
    Unknown,
}

#[derive(Deserialize)]
pub struct NoteForm {
    pub title: String,
    pub body: String,
    pub publisher: String,
    pub author: String,
    pub media_type: MediaType,
    pub year: usize,
    pub month: String,
    pub day: usize,
    pub externals: Vec<External>,
}
