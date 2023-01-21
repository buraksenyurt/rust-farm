mod data;
mod error;
mod model;
mod network;
mod security;
mod test;

use crate::data::db::{add_users_db, UsersDb};
use crate::error::handler::catch_rejection;
use crate::network::handler::{create_user, login};
use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

/*
   Örnek curl ifadeleri.

   Yeni kullanıcı kayıt etme.
   curl -X POST 'localhost:5555/register' -H "Content-Type: application/json" -d '{"username": "scoth", "password": "tiger@1234", "role": "admin"}'

   Login olma örneği
   curl -X POST 'localhost:5555/login' -H "Content-Type: application/json" -d '{"username": "scoth", "password": "tiger@1234"}'
*/

#[tokio::main]
async fn main() {
    // JWT token oluşturma işinde kullanılan Secret Key bilgisi environment'ten geliyor.
    // Uygulamada environment erişimi için aşağıdaki satır işletilir.
    dotenv::dotenv().ok();

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

    // login işlemleri için kullanılacak endpoint'e ait route tanımlamaları
    // HTTP Post ve JSON body kullanır.
    // İşlemler için handler.rs'teki login fonksiyonu çağırılır.
    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(add_users_db(db.clone()))
        .and_then(login);

    // route tanımlamaları çalışma zamanına eklenir.
    // CORS ayarına göre herkes erişebilir
    let routes = root
        .or(login_route)
        .or(register_route)
        .with(warp::cors().allow_any_origin())
        .recover(catch_rejection);

    info!("Sunucu dinlemede. http:://127.0.0.1:5555");
    // 127.0.0.1:5555 porttan sunucu açılır
    warp::serve(routes).run(([127, 0, 0, 1], 5555)).await;
}
