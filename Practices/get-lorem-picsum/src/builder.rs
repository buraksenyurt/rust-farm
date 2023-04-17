use crate::photo::Photo;
use reqwest;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum FetchError {
    Unsuccessful,
}

pub async fn get_photos() -> Result<Vec<Photo>, FetchError> {
    let response = reqwest::Client::new()
        .get("https://picsum.photos/v2/list?page=1&limit=10")
        .header("User-Agent", "Reqwest Rust Test")
        .send()
        .await;

    match response {
        Ok(r) => match r.status() {
            StatusCode::OK => match r.json::<Vec<Photo>>().await {
                Ok(parsed) => Ok(parsed),
                Err(e) => {
                    println!("{}", e);
                    Err(FetchError::Unsuccessful)
                }
            },
            _ => {
                println!("Status Code uygun değil");
                Err(FetchError::Unsuccessful)
            }
        },
        Err(e) => {
            println!("{}", e);
            Err(FetchError::Unsuccessful)
        }
    }
}
