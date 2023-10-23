use crate::feed::Feed;
pub fn load_feeds(feeds: &Vec<Feed>) {
    feeds.iter().for_each(|f| {
        let body = reqwest::blocking::get(f.url).unwrap().text().unwrap();
        // println!("{}", body);
        let feed_body = feed_rs::parser::parse(body.as_bytes()).unwrap();
        //println!("{:#?}", feed_body);
        println!(
            "{} ({}) - {}",
            f.title,
            feed_body.entries.len(),
            feed_body.updated.unwrap_or_default()
        );
        feed_body.entries.iter().enumerate().for_each(|(idx, e)| {
            println!(
                "{idx} - {} {}",
                e.title.clone().unwrap().content,
                e.links
                    .iter()
                    .map(|link| link.href.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        });
    });
}
