use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl MediaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaType::Gazete => "Gazete",
            MediaType::Dergi => "Dergi",
            MediaType::Dijital => "Dijital",
            MediaType::Kitap => "Kitap",
            MediaType::Podcast => "Podcast",
            MediaType::Medium => "Medium",
            MediaType::Unknown => "Unknown",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "Gazete" => MediaType::Gazete,
            "Dergi" => MediaType::Dergi,
            "Dijital" => MediaType::Dijital,
            "Kitap" => MediaType::Kitap,
            "Podcast" => MediaType::Podcast,
            "Medium" => MediaType::Medium,
            _ => MediaType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct External {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub publisher: String,
    pub author: String,
    pub media_type: MediaType,
    pub year: i64,
    pub month: String,
    pub day: i64,
    pub externals: Vec<External>,
    pub is_archived: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteInput {
    pub title: String,
    pub body: String,
    pub publisher: String,
    pub author: String,
    pub media_type: MediaType,
    pub year: i64,
    pub month: String,
    pub day: i64,
    pub externals: Vec<External>,
}
