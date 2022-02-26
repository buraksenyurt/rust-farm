/// HTTP Sunucu motoru ile ilgili çekirdek fonksiyonellikleri barındırır.
pub mod server {
    use crate::http::request::Request;
    use crate::http::response::{Response, StatusCode};
    use log::{error, info};
    use std::fmt::{Display, Formatter};
    use std::io::Read;
    use std::net::TcpListener;

    /// Sunucu bilgilerini taşıyan veri yapısı.
    pub struct Server<'a> {
        root: &'a str,
        port: u16,
        alias: &'a str,
    }

    impl<'a> Display for Server<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})->{}:{}", self.alias, self.root, self.port)
        }
    }

    impl<'a> Server<'a> {
        /// Yeni bir sunucu nesnesi örnekler.
        pub fn new(root: &'a str, port: u16, alias: &'a str) -> Self {
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
                                                info!(
                                                    "Request dönüşümü başarılı.{}",
                                                    r.to_string()
                                                );
                                                // İstemci tarafında bir Response yolluyoruz.
                                                // Şu an için dönüştürme operasyonunun başarılı olduğuna dair HTTP 200 bilgisi vermekteyiz.

                                                Response::new(
                                                        StatusCode::Ok,
                                                        Some(String::from("<h1>Would you like to play ping pong?</h1>")),
                                                    ).write(&mut stream).expect("Problem var!");
                                            }
                                            Err(e) => {
                                                error!("{:?}", e);
                                                Response::new(StatusCode::BadRequest, None)
                                                    .write(&mut stream)
                                                    .expect("Problem var!");
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("Stream okumada hata -> {}", e);
                                        Response::new(StatusCode::InternalServerError, None)
                                            .write(&mut stream)
                                            .expect("Problem var!");
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
}

/// HTTP için gerekli temel tipleri taşır
pub mod http {
    /// Ortak veri türlerini barındırır
    pub mod common {
        use crate::http::request::RequestError;
        use log::error;
        use std::str::FromStr;

        /// Kullanılabilecek HTTP metodlarını tutar
        #[derive(Debug, PartialEq)]
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
        pub struct Request<'a> {
            pub method: Command,
            pub path: &'a str,
            pub body: &'a str,
        }

        impl<'a> Request<'a> {
            /// Yeni bir HTTP Request oluşturmak için kullanılır.
            pub fn new(method: Command, path: &'a str, body: &'a str) -> Self {
                Request { method, path, body }
            }
        }

        impl<'a> Display for Request<'a> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}, {}, {}", self.method, self.path, self.body)
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

        /*
           TcpListener tarafından yakalanan stream içeriğini anlamlı hale getirmek için,
           Request türüne çevirmemiz lazım. Tabii uygun bir HTTP paketi söz konusu ise.
           Bu noktada dönüşüm sırasında hata olma ihtimalini de değerlendiren TryFrom trait'ini
           kullanabiliriz.

           Trait, listener'a gelen byte dizisini alıp Request türüne parse etme işlemini üstleniyor.

           Dönüşüm başarılı ise Request türü dönecek, değilse Error.
        */
        impl<'a> TryFrom<&'a [u8]> for Request<'a> {
            type Error = RequestError;

            fn try_from(value: &'a [u8]) -> Result<Request<'a>, Self::Error> {
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
                   Accept:
                   Content-Type: application/json
                   Content-Length: 36

                   {"message":"only one ping Vaseley."}

                   ya da

                   GET /query?word=red HTTP/1.1
                   Host: localhost:5555
                   User-Agent: curl/7.68.0
                   Accept:

                   Satır bazında gelen isteği ayrıştırıp örneğin ilk satırdan HTTP metodu,
                   path, query string son kısımdan JSON content vs almamız mümkün.
                */

                // Gelen içeriği satır bazında bir vector içinde topluyoruz.
                let parts: Vec<&str> = package.split('\n').collect();
                for p in &parts {
                    info!("Part -> {}", p);
                }

                let first_row = parts[0];
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

                // from_str trait'ini yukarıda Command veri yapısına uyarlamıştık
                let c = Command::from_str(cmd[0])?;
                // Http metodunun Post olması halinde JSON içeriğini almayı da deneyebiliriz.
                let body = match c {
                    Command::Post => parts[parts.len() - 1].trim(),
                    _ => "",
                };
                Ok(Self::new(c, cmd[1], body))
            }
        }
    }

    /// Response ile ilgili enstrümanları barındırır.
    pub mod response {
        use std::fmt::{Display, Formatter};
        use std::io::{Result, Write};
        use std::net::TcpStream;

        pub struct Response {
            status_code: StatusCode,
            body: Option<String>,
        }

        impl Response {
            pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
                Response { status_code, body }
            }

            pub fn write(&self, stream: &mut TcpStream) -> Result<()> {
                let body = match &self.body {
                    Some(b) => b,
                    None => "",
                };
                write!(
                    stream,
                    "HTTP/1.1 {} \r\n\r\n{}",
                    self.status_code.to_string(),
                    body
                )
            }
        }

        /// Birkaç HTTP statü kodunu tutan enum sabiti
        #[derive(Copy, Clone)]
        pub enum StatusCode {
            Ok = 200,
            BadRequest = 400,
            Unauthorized = 401,
            NotFound = 404,
            InternalServerError = 500,
        }

        impl Display for StatusCode {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                /*
                    enum sabitindeki sayısal değeri almak için dereference işlemi uyguladık.
                    Ancak bunu yaparken StatusCode'un Copy ve Clone trait'lerini uygulamış olması
                    gerekiyor. Nitekim buradaki move işlemi için kopyalama gerekiyor.
                */
                let code = *self as u16;
                match self {
                    Self::Ok => write!(f, "{} Ok", code),
                    Self::BadRequest => write!(f, "{} Bad request", code),
                    Self::Unauthorized => write!(f, "{} Unauthorized", code),
                    Self::NotFound => write!(f, "{} Not found", code),
                    Self::InternalServerError => write!(f, "{} Internal server error", code),
                }
            }
        }
    }
}
