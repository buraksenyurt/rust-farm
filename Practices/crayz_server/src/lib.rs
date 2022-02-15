/// HTTP Sunucu motoru ile ilgili çekirdek fonksiyonellikleri barındırır.
pub mod server {
    use log::{error, info};
    use std::fmt::{Display, Formatter};
    use std::io::Read;
    use std::net::TcpListener;

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

        fn address(&self) -> String {
            format!("{}:{}", &self.root, &self.port)
        }

        /// Sunucuyu dinleme modunda başlatır
        pub fn run(self) {
            /*
               Tipik bir HTTP sunucusu başlatıldığında sonsuz bir döngüde talep dinler.
               Dolayısıyla run fonksiyonu sonlandığında self ile ifade edilen ve
               sahiliği(ownership)'i alınan Server nesnesinin deallocate edilmesinde yarar vardır.
               Bu sebepten &self yerine self kullandık.
            */
            info!("{} başlatılıyor...", self.address());
            /*
                Sunucuyu hazırlamak için TcpListener'ın bind fonksiyonu kullanılır.
            */
            let listener = TcpListener::bind(&self.address());
            match listener {
                Ok(l) => {
                    info!("{} başlatıldı...", self.to_string());
                    // Gelen talepleri sonsuz bir döngüde dinleyebiliriz.
                    loop {
                        // Gelen yeni bağlantıları match ifadesi ile kontrol altına alıyoruz.
                        match l.accept() {
                            Ok((mut stream, addrees)) => {
                                info!("İstemci -> {}", addrees.to_string());

                                /*
                                    İstemciden gelen talebi belli bir boyuttaki dizi içerisine almalıyız
                                    read trait'inin TcpStream için implemente edilmiş versiyonu,
                                    gelen içeriği mutable bir dizi içerisine yazmak üzere tasarlanmış.
                                    Başlangıç için 1024 elemanlı bir array göz önüne alabiliriz.
                                */
                                let mut buffer = [0 as u8; 1024];
                                match stream.read(&mut buffer) {
                                    Ok(l) => {
                                        let msg = String::from_utf8(buffer[0..l].to_vec());
                                        info!("Gelen bilgi -> {:?}", msg.unwrap());

                                        //TODO Mesaj boyutunun belli bir değerin üstünde olmamasını garanti edelim.
                                        //TODO Gelen istekler Request veri yapısına dönüştürülmeli
                                    }
                                    Err(e) => {
                                        error!("Stream okumada hata -> {}", e);
                                    }
                                }

                            }
                            Err(e) => {
                                error!("Bağlantı sağlanırken bir hata oluştu. Hata detayı -> {}", e)
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Sunucu başlatılamadı. Hata detayı -> {}", e);
                }
            }
        }
    }

    impl Display for Server {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})->{}:{}", self.alias, self.root, self.port)
        }
    }
}

/// HTTP için gerekli temel tipleri taşır
pub mod http {
    /// Ortak veri türlerini barındırır
    pub mod common {
        /// Kullanılabilecek HTTP metodlarını tutar
        #[derive(Debug)]
        pub enum Method {
            Get(Option<String>),
            // query string saklayabiliriz
            Post,
            Put,
            Delete,
        }
    }

    /// Request ile ilgili enstrümanları barındırır.
    pub mod request {
        use super::common::Method;
        use std::fmt::{Display, Formatter};
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
    }
}
