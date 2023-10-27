use crate::reader::load_feeds_from_file;
use dotenv::dotenv;
use std::env;

mod feed;
mod reader;

fn main() {
    dotenv().ok().expect("Environment dosyası okunamadı");
    let source_path = env::var("DataSource").expect("Env dosyasında datasource içeriği yok");
    let feeds = load_feeds_from_file(source_path);
    println!("Feed count {}", feeds.len());

    // let feeds.dat = vec![
    //     Feed::new("Tech Crunch", "https://techcrunch.com/feed/"),
    //     Feed::new("Hacker News", "https://news.ycombinator.com/rss"),
    // ];

    reader::get(&feeds, 5);
}
