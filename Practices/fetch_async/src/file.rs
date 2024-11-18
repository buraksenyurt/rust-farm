use std::io::Result;

pub async fn save_to_file(file_name: &str, data: &str) -> Result<()> {
    tokio::fs::write(file_name, data).await
}
