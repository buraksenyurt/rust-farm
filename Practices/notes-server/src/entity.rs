use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    id: usize,
    title: String,
    body: String,
    publisher: String,
    author: String,
    #[serde(rename = "mediaType")]
    media_type: MediaType,
    year: usize,
    month: String,
    day: usize,
    #[serde(default)]
    externals: Option<Vec<External>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct External {
    title: String,
    url: String,
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
