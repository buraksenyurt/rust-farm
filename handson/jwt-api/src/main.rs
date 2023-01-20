mod data;
mod model;
mod network;

use crate::data::db::UsersDb;
use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

#[tokio::main]
async fn main() {
    // loglama ayarlarını içeren dosyayı yüklüyoruz
    log4rs::init_file("log_config.yml", Default::default()).expect("Config dosyası bulunamadı");

    info!("Veritabanı örneği hazırlanıyor...");
    let db: UsersDb = Arc::new(Mutex::new(HashMap::new()));

    info!("Sunucu çalıştırılıyor...");
    // Başlangıç için kullanılacak bir wellcome sayfası
    let root = warp::path::end().map(|| "Wellcome page");
    // CORS ayarlaması. Herhangi bir kaynaktan gelinebilir
    let routes = root.with(warp::cors().allow_any_origin());
    info!("Sunucu dinlemede. http:://127.0.0.1:5555");
    // 127.0.0.1:5555 porttan sunucu açılır
    warp::serve(routes).run(([127, 0, 0, 1], 5555)).await;
}
