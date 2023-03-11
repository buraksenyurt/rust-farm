use kafka::producer::{Producer, Record, RequiredAcks};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let call_response = fetch_articles().await?;
    println!("{} makale bulundu", call_response.articles.len());
    send_to_kafka("technews", call_response.articles);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Article {
    pub title: String,
    pub description: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct News {
    pub articles: Vec<Article>,
}

async fn fetch_articles() -> Result<News, reqwest::Error> {
    let client = reqwest::Client::new();
    let request = client
        .get("https://saurav.tech/NewsAPI/top-headlines/category/technology/in.json")
        .build()
        .unwrap();
    let json_response = client.execute(request).await?.json::<News>().await?;
    Ok(json_response)
}

fn send_to_kafka(topic: &str, payload: Vec<Article>) {
    let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    for article in payload {
        let buffer = serde_json::to_string(&article).unwrap();
        producer
            .send(&Record::from_value(topic, buffer.as_bytes()))
            .unwrap();
    }
}
