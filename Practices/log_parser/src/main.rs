use std::fmt::{Display, Formatter};
use std::fs;
use std::io::Error;

/*
   Bu senaryoda bir log dosyasındaki türleri ayrıştırıp farklı bir dosyaya yazma
   fonksiyonelliği üzerinde duruluyor. Özellikle dosya okuma ve yazma işlemleri Result<T,E>
   döndüğünden, olası hata durumlarını hangi şekillerde ele alabileceğimize bakılıyor.

   Aşağıdaki senaryoda kaynak log doyasının olmaması veya yazma sırasında alınacak hatalar
   üst fonksiyonelliği yönlendirilebilecek türden hatalar. Nitelim bir workaround'umuz yok.
   Bu nedenle ?(try) operatörü kullanımı daha idiomatic bir yaklaşım olacaktır.

   match statement : Eğer hata durumları ile anlamlı bir şekilde ilgilenilecekse. Örneğin
   bir settings dosyasında bazı ayarları aldığımızı var sayalım. Eğer okuyamazsak varsayılan
   bazı ayarları devreye alabilirizi. Burada match kullanarak söz konusu kod blogları çalıştırılabilir.

   unwrap, expect : Çabuk debug yapmak veya hata durumunda programın çakılması istendiğinde.

   ?(try) operator : Hatayı ele almanın bir yolu olmadığında bunu üst fonksiyona
   bildirmek gerektiğinde.

*/
fn main() -> Result<(), Error> {
    // ?(try) operatörünün kullanımı
    let logs = fs::read_to_string("system.log")?;
    let error_logs = map_logs(logs.as_str(), LogType::Error);
    fs::write(
        format!("{}s.log", LogType::Error.to_string().to_lowercase()),
        error_logs.join("\n"),
    )?;

    Ok(())

    // // Daha kontrollü yaklaşım
    // let mut partial_logs = vec![];
    //
    // match fs::read_to_string("system.log") {
    //     Ok(content) => {
    //         partial_logs = map_logs(content.as_str(), LogType::Error);
    //         match fs::write(
    //             format!("{}s.log", LogType::Error.to_string().to_lowercase()),
    //             partial_logs.join("\n"),
    //         ) {
    //             Ok(_) => {
    //                 println!("Partial logs writing to file...");
    //             }
    //             Err(e) => {
    //                 println!("{}", e);
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    // }
    //
    // // println!("{:#?}", partial_logs); // Lifetime error
    //
    // Ok(())

    // // expect fonksiyonu kullanımı
    // let logs = fs::read_to_string("system.log").expect("Failed to read from system logs.");
    // let error_logs = map_logs(logs.as_str(), LogType::Error);
    // fs::write(
    //     format!("{}s.log", LogType::Error.to_string().to_lowercase()),
    //     error_logs.join("\n"),
    // )
    // .expect("Failed to write partial logs");
    //
    // Ok(())
}

fn map_logs(context: &str, log_type: LogType) -> Vec<&str> {
    let lines = context.split("\n");
    let mut result = vec![];
    let prefix = log_type.to_string();
    for line in lines {
        if line.starts_with(&prefix) {
            result.push(line);
        }
    }
    result
}

#[derive(Debug)]
enum LogType {
    Error,
    Warn,
    Info,
}

impl Display for LogType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogType::Error => write!(f, "ERROR"),
            LogType::Warn => write!(f, "WARN"),
            LogType::Info => write!(f, "INFO"),
        }
    }
}
