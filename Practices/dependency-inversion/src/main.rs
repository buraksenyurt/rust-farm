use crate::bad_practice::{HtmlParser, WebScraper};

mod bad_practice;

fn main() {
    // Without Dependency Inversion

    let crawler = WebScraper {
        parser: HtmlParser {},
    };
    let data = crawler.apply("https://www.buraksenyurt.com");
    data.iter().for_each(|c| println!("{c}"));

    // Without Dependency Inversion
}
