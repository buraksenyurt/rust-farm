use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Photo {
    pub id: String,
    pub author: String,
    pub width: u32,
    pub height: u32,
    pub url: String,
    pub download_url: String,
}
