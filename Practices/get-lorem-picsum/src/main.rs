use reqwest;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let response = reqwest::Client::new()
        .get("https://picsum.photos/v2/list?page=1&limit=10")
        .header("User-Agent", "Reqwest Rust Test")
        .send()
        .await;

    // println!("{}", response.unwrap().status());

    match response {
        Ok(r) => match r.status() {
            StatusCode::OK => match r.json::<Vec<Photo>>().await {
                Ok(parsed) => {
                    println!("{:#?}", parsed)
                }
                Err(e) => {
                    println!("{}", e)
                }
            },
            _ => println!("Status Code uygun değil"),
        },
        Err(e) => {
            println!("{}", e)
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Photo {
    pub id: String,
    pub author: String,
    pub width: u32,
    pub height: u32,
    pub url: String,
    pub download_url: String,
}
