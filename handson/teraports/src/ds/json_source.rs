use crate::ds::DataSource;
use serde::de::DeserializeOwned;
use tokio::fs;

pub struct JsonSource {
    pub file_name: String,
}

impl DataSource for JsonSource {
    async fn load_data<T: DeserializeOwned>(&self) -> T {
        let content = fs::read_to_string(&self.file_name)
            .await
            .expect("Error reading JSON file");
        serde_json::from_str(&content).expect("Error deserializing JSON data")
    }
}
