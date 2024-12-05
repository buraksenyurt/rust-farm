use crate::ds::TabularLoader;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct CsvDataSource {
    pub file_name: String,
}

impl TabularLoader for CsvDataSource {
    async fn load_data<T: Serialize + DeserializeOwned + Send>(&self) -> Vec<T> {
        let content = tokio::fs::read_to_string(&self.file_name)
            .await
            .expect("Error reading tabular data file");
        let mut reader = csv::Reader::from_reader(content.as_bytes());
        reader
            .deserialize()
            .map(|result| result.expect("Error deserializing tabular row"))
            .collect()
    }
}
