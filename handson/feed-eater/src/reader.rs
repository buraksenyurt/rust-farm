use crate::feed::Feed;
pub fn load_feeds(feeds: &Vec<Feed>) {
    feeds.iter().for_each(|f| {
        println!("{}", f.title);

        let body = reqwest::blocking::get(f.url).unwrap().text().unwrap();
        println!("{}", body);
    });
}
