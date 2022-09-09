use crate::model::database::init;
use crate::web::run_web_server;
use std::env;
use std::sync::Arc;

mod model;
mod security;
mod web;

// Web sunucusunun kullanacağı varsayılan disk yolu ve port bilgisini birer sabit ile tutabiliriz.
// iis altındaki bir web site folder olarak düşünebiliriz.
const DEFAULT_WEB_FOLDER: &'static str = "www/";
const DEFAULT_PORT: u16 = 5001;

#[tokio::main]
async fn main() {
    env_logger::init();
    // Web klasörünün adını parametre olarak komut satırından alabiliriz
    // eğer bir şey girilmeze varsayılan path kullanılır
    let mut args: Vec<String> = env::args().collect();
    let web_folder = args.pop().unwrap_or_else(|| DEFAULT_WEB_FOLDER.to_string());
    let port = DEFAULT_PORT;

    // veri tabanını başlatıyoruz ki ilerde sayfa içeriğini buradan karşılayacağız
    let db = init().await.expect("Veritabanı başlatılamadı");
    // Atomik referasn sayacı oluşturuyoruz. NEDEN???
    let db = Arc::new(db);

    println!("Web sunucusu başlatılacak");

    match run_web_server(&web_folder, port, db).await {
        Ok(_) => println!("Web sunucusu başlatıldı"),
        Err(e) => println!("Bir hata oluştu {:?}", e),
    }
}
