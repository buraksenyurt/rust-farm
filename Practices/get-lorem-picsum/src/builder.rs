use crate::photo::Photo;
use reqwest;
use reqwest::StatusCode;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub enum ProcessError {
    Unsuccessful,
    SaveError,
}

pub async fn get_photos() -> Result<Vec<Photo>, ProcessError> {
    let response = reqwest::Client::new()
        .get("https://picsum.photos/v2/list?page=1&limit=10")
        .header("User-Agent", "Reqwest Client")
        .send()
        .await;

    match response {
        Ok(r) => match r.status() {
            StatusCode::OK => match r.json::<Vec<Photo>>().await {
                Ok(parsed) => Ok(parsed),
                Err(e) => {
                    println!("{}", e);
                    Err(ProcessError::Unsuccessful)
                }
            },
            _ => {
                println!("Status Code uygun değil");
                Err(ProcessError::Unsuccessful)
            }
        },
        Err(e) => {
            println!("{}", e);
            Err(ProcessError::Unsuccessful)
        }
    }
}

pub async fn write_to_file(photo: &Photo) {
    let file_name = photo.create_file_name();
    let mut file = File::create(format!("./Photos/{}", file_name)).expect("Dosya oluşturma hatası");

    let content = reqwest::Client::new()
        .get(&photo.download_url)
        .header("User-Agent", "Reqwest Client")
        .send()
        .await
        .expect("Veri gönderim hatası")
        .bytes()
        .await
        .expect("byte içeriği okuma hatası");

    let mut pos = 0;
    while pos < content.len() {
        let bytes_written = file
            .write(&content[pos..])
            .expect("byte içeriği yazma hatası");
        pos += bytes_written;
    }
}
