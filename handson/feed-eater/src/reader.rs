use crate::feed::Feed;
use feed_rs::model::Text;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_feeds_from_file(source: String) -> Vec<Feed> {
    let file = File::open(source).expect("Dosya açma hatası");
    let reader = BufReader::new(file);
    let mut feeds = Vec::new();

    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.split(',').collect();
        //println!("Parts len {}",parts.len());
        if parts.len() == 2 {
            let title = parts[0].trim().to_string();
            let url = parts[1].trim().to_string();
            //println!("{title},{url}");
            let feed = Feed::new(title, url);
            //println!("{}", feed.to_string());
            feeds.push(feed);
        }
    }

    feeds
}

pub fn get(feeds: &[Feed], count: u8) {
    feeds.iter().for_each(|f| {
        //println!("Request -> {}", f.url.as_str());
        let body = reqwest::blocking::get(f.url.as_str())
            .unwrap()
            .text()
            .unwrap();
        // println!("{}", body);
        let feed_body = feed_rs::parser::parse(body.as_bytes()).unwrap();
        //println!("{:#?}", feed_body);
        println!(
            "\n*** {} ({}) - {} ***\n",
            f.title,
            feed_body.entries.len(),
            feed_body.updated.unwrap_or_default()
        );
        feed_body
            .entries
            .iter()
            .enumerate()
            .take(count as usize)
            .for_each(|(idx, e)| {
                let title = get_short(e.title.clone(), 50);
                //let summary = get_short(e.summary.clone(), 100);
                println!(
                    "\n{idx} - {}... [{}]\n{}\n", //{}...",
                    title,
                    e.published.unwrap_or_default(),
                    e.links
                        .iter()
                        .map(|link| link.href.clone())
                        .collect::<Vec<String>>()
                        .join(", "),
                    //summary
                );
            });
    });
}

fn get_short(e: Option<Text>, size: usize) -> String {
    let e_clone = e.unwrap().clone();
    if e_clone.content.len() > size {
        e_clone.content[0..size].parse::<String>().unwrap()
    } else {
        e_clone.content
    }
}
