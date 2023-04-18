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

impl Photo {
    pub fn create_file_name(&self) -> String {
        format!(
            "{}_{}_{}_{}.jpg",
            self.author.replace(' ', "_"),
            self.width,
            self.height,
            self.id
        )
    }
}
