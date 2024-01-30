// Önce abstraction veri yapısını hazırlayalım
// Mevzu bir içeriği HTML, JSON, XML parse edecek servis için ortak davranış tanımlamak
pub trait Parser {
    fn parse(&self, source: &str) -> Vec<String>;
}

// Şimdi buna göre low-level modülleri aşağıdaki gibi tasarlayabiliriz
pub struct JsonParser;

impl Parser for JsonParser {
    fn parse(&self, source: &str) -> Vec<String> {
        println!("{source} üstünde JSON parse işlemi uygulanıyor");
        vec![]
    }
}

pub struct XmlParser;

impl Parser for XmlParser {
    fn parse(&self, source: &str) -> Vec<String> {
        println!("{source} üstünde XML parse işlemi uygulanıyor");
        vec![]
    }
}

pub struct SmartHtmlParser;

impl Parser for SmartHtmlParser {
    fn parse(&self, source: &str) -> Vec<String> {
        println!("{source} üstünde HTML parse işlemi uygulanıyor");
        vec![]
    }
}

pub struct WebCrawler<'a> {
    pub parser: Box<dyn Parser + 'a>, // Abstraction üzerinden depedency çözümlemesi
}

impl<'a> WebCrawler<'a> {
    fn fetch_data(&self, url: &str) -> String {
        println!("{url} adresinden kaynak çekiliyor");
        String::new()
    }
    pub fn apply(&self, url: &str) -> Vec<String> {
        let data = self.fetch_data(url);
        // WebCrawler'a hangi parser bileşenini verdiysek onun tarzında parse işlemi yapılır
        self.parser.parse(&data)
    }
}
