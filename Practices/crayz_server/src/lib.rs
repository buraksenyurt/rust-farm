/// HTTP Sunucu motoru ile ilgili çekirdek fonksiyonellikleri barındırır.
pub mod server {
    use crate::http::request::Request;
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
                                let mut buffer = [0_u8; 1024];
                                match stream.read(&mut buffer) {
                                    Ok(l) => {
                                        let msg = String::from_utf8(buffer[0..l].to_vec());
                                        info!("Gelen bilgi -> {:?}", msg.unwrap());
                                        // Request tipini try_from ile donatmıştık. Dolayısıyla gelen mesajı Request türüne çevirmeyi deneyebiliriz.
                                        let converted_msg = Request::try_from(&buffer[..]);
                                        match converted_msg {
                                            Ok(r) => {
                                                info!("Request dönüşümü başarılı.{}", r.to_string())
                                            }
                                            Err(e) => {
                                                error!("{:?}", e)
                                            }
                                        }
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
        use crate::http::request::RequestError;
        use log::error;
        use std::str::FromStr;

        /// Kullanılabilecek HTTP metodlarını tutar
        #[derive(Debug)]
        pub enum Command {
            Get,
            Post,
            Put,
            Delete,
        }

        /// string'ten Command elde etme davranışını uygular
        impl FromStr for Command {
            type Err = RequestError;

            /*
            from_str trait'ini Command veri yapısı için yeniden programladık.
            Böylece bir string'i ele alıp uygun Command nesnesini elde edebiliriz.
            Geçerli bir command nesnesi değilse de RequestError döndürmekteyiz.
            Bu fonksiyonu Request veri yapısında ele aldığımız TryFrom trait içerisinde kullanacağız.
             */
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "GET" => Ok(Self::Get),
                    "POST" => Ok(Self::Post),
                    "DELETE" => Ok(Self::Delete),
                    "PUT" => Ok(Self::Put),
                    _ => {
                        error!("Geçersiz bir metot geldi.");
                        Err(RequestError::Command)
                    }
                }
            }
        }
    }

    /// Request ile ilgili enstrümanları barındırır.
    pub mod request {
        use crate::http::common::Command;
        use log::{error, info};
        use std::fmt::{Display, Formatter};
        use std::str;
        use std::str::FromStr;
        use thiserror::Error;

        /// HTTP Request içeriğini tutar.
        pub struct Request {
            pub method: Command,
            pub path: String,
        }

        impl Request {
            /// Yeni bir HTTP Request oluşturmak için kullanılır.
            pub fn new(method: Command, path: String) -> Self {
                Request { method, path }
            }
        }

        impl Display for Request {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}, {}", self.method, self.path)
            }
        }

        /*
           TcpListener tarafından yakalanan stream içeriğini anlamlı hale getirmek için,
           Request türüne çevirmemiz lazım. Tabii uygun bir HTTP paketi söz konusu ise.
           Bu noktada dönüşüm sırasında hata olma ihtimalini de değerlendiren TryFrom trait'ini
           kullanabiliriz.

           Trait, listener'a gelen byte dizisini alıp Request türüne parse etme işlemini üstleniyor.

           Dönüşüm başarılı ise Request türü dönecek, değilse Error.
        */
        impl TryFrom<&[u8]> for Request {
            type Error = RequestError;

            fn try_from(value: &[u8]) -> Result<Self, RequestError> {
                /*
                    Pattern matching kullanımı yerine ? operatörü ile işi kısaltabiliriz.

                    from_utf8 fonksiyonu eğer gelen parametreyi çözümleyemezse ParseError verir.
                    or fonksiyonunda, ParseError olması halinde kendi Encoding error nesnemizi
                    döndüreceğimizi belirtiyoruz.

                    ? operatörü encoding sorunu yoksa, çözümlenmiş içeriğin package nesnesine
                    alınmasını sağlar.
                */

                let package = str::from_utf8(value).or(Err(RequestError::Encoding))?;

                /*
                   Gelen HTTP paketi satır satır akacaktır. Örneğin aşağıdaki gibi,

                   POST /movies/ HTTP/1.1
                   Host: localhost:5555
                   User-Agent: curl/7.68.0
                   Accept: *//*
                   Content-Type: application/json
                   Content-Length: 36

                   {"message":"only one ping Vaseley."}

                   ya da

                   GET /query?word=red HTTP/1.1
                   Host: localhost:5555
                   User-Agent: curl/7.68.0
                   Accept: *//*

                   Satır bazında gelen isteği ayrıştırıp örneğin ilk satırdan HTTP metodu,
                   path, query string son kısımdan JSON content vs almamız mümkün.
                */

                // Gelen içeriği satır bazında bir vector içinde topluyoruz.
                let parts: Vec<&str> = package.split('\n').collect();
                for p in &parts {
                    info!("Part -> {}", p);
                }

                let first_row = parts[0].to_string();
                let cmd: Vec<&str> = first_row.split(' ').collect();

                /*
                    Eğer sadece HTTP paketlerini ele alacaksak ilk satırda en azından

                    GET /authors/ HTTP/1.1

                    benzeri bir içerik olmalı.
                    Dolayısıyla ilk satırın split edilmiş hali 3 eleman olmalı.
                */
                if cmd.len() != 3 {
                    return Err(RequestError::Invalid);
                }

                let protocol = cmd[2];
                if !protocol.contains("HTTP") {
                    return Err(RequestError::Protocol);
                }

                //TODO Eğer metot POST ise JSON içeriğini alalım

                // from_str trait'ini yukarıda Command veri yapısına uyarlamıştık
                let c = Command::from_str(cmd[0])?;
                Ok(Request::new(c, cmd[1].to_string()))
            }
        }

        // Hata durumlarını tutan enum sabitimizde işleri kolaylaştıran thiserror paketini kullandık.
        /// Request dönüşümlerindeki olası hata durumlarını tutar.
        #[derive(Debug, Error)]
        pub enum RequestError {
            #[error("Paket geçersiz")]
            Invalid,
            #[error("Geçersiz HTTP komutu")]
            Command,
            #[error("Protokol geçersiz")]
            Protocol,
            #[error("Sorunlu encoding")]
            Encoding,
            #[error("HTTP ile uyumlu değil")]
            NotCompatible,
        }
    }
}
