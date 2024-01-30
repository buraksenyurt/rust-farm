use crate::bad_practice::{HtmlParser, WebScraper};
use crate::refactored::{JsonParser, SmartHtmlParser, WebCrawler, XmlParser};

mod bad_practice;
mod refactored;

fn main() {
    // Without Dependency Inversion

    let crawler = WebScraper {
        parser: HtmlParser {},
    };
    let data = crawler.apply("https://www.buraksenyurt.com/index.html");
    data.iter().for_each(|c| println!("{c}"));

    // Without Dependency Inversion

    // With Dependency Inversion

    let json_parser = JsonParser;
    let crawler = WebCrawler {
        parser: Box::new(json_parser),
    };
    let _data = crawler.apply("https://somewhere.com/games.json");

    let xml_parser = XmlParser;
    let crawler = WebCrawler {
        parser: Box::new(xml_parser),
    };
    let _data = crawler.apply("https://somewhere.com/stats.xml");

    let html_parser = SmartHtmlParser;
    let crawler = WebCrawler {
        parser: Box::new(html_parser),
    };
    let _data = crawler.apply("https://www.buraksenyurt.com/index.html");

    // With Dependency Inversion
}
