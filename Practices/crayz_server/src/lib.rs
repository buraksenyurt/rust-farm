use log::info;
use std::fmt::{Display, Formatter};

/// Sunucu bilgilerini taşıyan veri yapısı.
pub struct Server {
    root: String,
    port: u16,
    alias: String,
}

impl Server {
    /// Yeni bir sunucu nesnesi örnekler.
    pub fn new(root: String, port: u16, alias: String) -> Self {
        Server { root, port, alias }
    }

    /// Sunucuyu dinleme modunda başlatır
    pub fn run(self) {
        /*
           Tipik bir HTTP sunucusu başlatıldığında sonsuz bir döngüde talep dinler.
           Dolayısıyla run fonksiyonu sonlandığında self ile ifade edilen ve
           sahiliği(ownership)'i alınan Server nesnesinin deallocate edilmesinde yarar vardır.
           Bu sebepten &self yerine self kullandık.
        */
        info!("{} başlatılıyor...", self.to_string());
        //TODO Listener yazılacak
        info!("{} başlatıldı...", self.to_string());
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})->{}:{}", self.alias, self.root, self.port)
    }
}

/// Kullanılabilecek HTTP metodlarını tutar
#[derive(Debug)]
pub enum Method {
    Get(Option<String>), // query string saklayabiliriz
    Post,
    Put,
    Delete,
}

/// HTTP Request içeriğini tutar.
pub struct Request {
    pub method: Method,
    pub path: String,
}

impl Request {
    /// Yeni bir HTTP Request oluşturmak için kullanılır.
    pub fn new(method: Method, path: String) -> Self {
        Request { method, path }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP {:?}, {}", self.method, self.path)
    }
}
