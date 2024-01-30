// Bu low-level modüle görevini üstlenen veri yapısı
// HTML içeriklerini parse etme işini icra ediyor
pub struct HtmlParser {}

impl HtmlParser {
    pub fn parse(&self, content: &str) -> Vec<String> {
        println!("HTML Parse işlemi uygulanıyor.\nİçerik;\n{content}");
        vec![
            "html".to_string(),
            "head".to_string(),
            "body".to_string(),
            "p1".to_string(),
        ]
    }
}

// High-Level modülü görevini üstlenen WebScraper veri yapısı
pub struct WebScraper {
    // Burada tightly-coupled ilişki kurmuş olduk. WebScrapper doğrudan HtmlParser'a bağımlı
    pub parser: HtmlParser,
}

impl WebScraper {
    fn fetch_html_content(&self, url: &str) -> String {
        println!("Sembolik olarak {url} adresinin html içeriği çekiliyor");
        String::from("<html><head>It's a lovely day!</head><body><p1>Game list</p1></body></html>")
    }
    pub fn apply(&self, url: &str) -> Vec<String> {
        let content = self.fetch_html_content(url);
        self.parser.parse(&content)
    }
}
