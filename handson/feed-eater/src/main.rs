use crate::feed::Feed;

mod feed;
mod reader;

fn main() {
    let feeds = vec![
        Feed::new("Tech Crunch", "https://techcrunch.com/feed/"),
        Feed::new("Hacker News", "https://news.ycombinator.com/rss"),
    ];
    reader::get(&feeds, 5);
}
