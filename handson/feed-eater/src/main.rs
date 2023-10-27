use crate::reader::load_feeds_from_file;
use dotenv::dotenv;
use std::env;

mod feed;
mod reader;

fn main() {
    dotenv().expect("Environment dosyası okunamadı");
    let source_path = env::var("DATASOURCE").expect("Env dosyasında datasource içeriği yok");
    let feeds = load_feeds_from_file(source_path);

    // let feeds.dat = vec![
    //     Feed::new("Tech Crunch", "https://techcrunch.com/feed/"),
    //     Feed::new("Hacker News", "https://news.ycombinator.com/rss"),
    // ];

    reader::get(&feeds, 5);
}
