mod data;
mod model;
mod network;

use crate::data::db::{add_users_db, UsersDb};
use crate::network::handler::create_user;
use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

/*
    Örnek curl ifadeleri.

    Yeni kullanıcı kayıt etme.
    curl -X POST 'localhost:5555/register' -H "Content-Type: application/json" -d '{"username": "scoth", "password": "tiger@1234", "role": "admin"}'
 */

#[tokio::main]
async fn main() {
    // loglama ayarlarını içeren dosyayı yüklüyoruz
    log4rs::init_file("log_config.yml", Default::default()).expect("Config dosyası bulunamadı");

    info!("Veritabanı örneği hazırlanıyor...");
    let db: UsersDb = Arc::new(Mutex::new(HashMap::new()));

    info!("Sunucu çalıştırılıyor...");
    // Başlangıç için kullanılacak bir wellcome sayfası
    let root = warp::path::end().map(|| "Wellcome page");

    // Kullanıcı oluşturma(Register) işini üstlenen route tanımlamaları.
    // HTTP Post kullanılacak. JSON formatında mesaj kabul edilecek.
    // add_users_db ile elde edilen sahte veritabanı işin içerisine katılacak.
    // Kullanıcı oluşturma işini de create_user fonksiyonu gerçekleştirecek.
    let register_route = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and(add_users_db(db.clone()))
        .and_then(create_user);

    // route tanımlamaları çalışma zamanına eklenir.
    // CORS ayarına göre herkes erişebilir
    let routes = root
        .or(register_route)
        .with(warp::cors().allow_any_origin());

    info!("Sunucu dinlemede. http:://127.0.0.1:5555");
    // 127.0.0.1:5555 porttan sunucu açılır
    warp::serve(routes).run(([127, 0, 0, 1], 5555)).await;
}
