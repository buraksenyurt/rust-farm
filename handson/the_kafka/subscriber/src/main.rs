use kafka::consumer::{Consumer, FetchOffset};
use serde::Deserialize;
use std::ops::Deref;

fn main() {
    let mut consumer = Consumer::from_hosts(vec!["localhost:9092".to_string()])
        .with_topic("technews".to_string())
        .with_fallback_offset(FetchOffset::Earliest)
        .create()
        .unwrap();

    loop {
        for msg_set in consumer.poll().unwrap().iter() {
            for msg in msg_set.messages() {
                let value = String::from_utf8_lossy(msg.value);
                let article: Article = serde_json::from_str::<Article>(value.deref()).unwrap();
                println!("{} ({})", article.title,article.url);
            }
            let _ = consumer.consume_messageset(msg_set);
        }
        consumer.commit_consumed().unwrap();
    }
}

#[derive(Debug, Deserialize)]
struct Article {
    pub title: String,
    pub description: String,
    pub url: String,
}
