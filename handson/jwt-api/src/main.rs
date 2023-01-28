mod data;
mod error;
mod model;
mod network;
mod security;
mod test;

use crate::data::db::{add_with_db, create_conn_pool, init_db};
use crate::error::handler::catch_rejection;
use crate::network::handler::{create_user, get_categories, get_salary_stats, login};
use crate::security::auditer::with_auth;
use crate::security::role::Role;
use log::info;
use warp::Filter;

#[tokio::main]
async fn main() {
    // JWT token oluşturma işinde kullanılan Secret Key bilgisi environment'ten geliyor.
    // Uygulamada environment erişimi için aşağıdaki satır işletilir.
    dotenv::dotenv().ok();

    // loglama ayarlarını içeren dosyayı yüklüyoruz
    log4rs::init_file("log_config.yml", Default::default()).expect("Config dosyası bulunamadı");

    info!("Veritabanı örneği hazırlanıyor...");
    let conn_pool = create_conn_pool().expect("bağlantı havuzu oluşturulamadı");
    init_db(&conn_pool).await.expect("veritabanı başlatılamadı");

    // let (client, connection) =
    //     tokio_postgres::connect("host=localhost user=scoth password=tiger", NoTls)
    //         .await
    //         .unwrap();
    //
    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         info!("Postgresql bağlantı hatası: {}", e);
    //     }
    // });

    //let db: UsersDb = Arc::new(Mutex::new(HashMap::new()));

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
        .and(add_with_db(conn_pool.clone()))
        .and_then(create_user);

    // login işlemleri için kullanılacak endpoint'e ait route tanımlamaları
    // HTTP Post ve JSON body kullanır.
    // İşlemler için handler.rs'teki login fonksiyonu çağırılır.
    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(add_with_db(conn_pool.clone()))
        .and_then(login);

    // stats_route tanımında sadece Admin yetkisine sahip olanların
    // gidebileceği bir yönlendirme bildirimi söz konusu.
    let stats_route = warp::path("stats")
        .and(warp::get())
        .and(with_auth(Role::Admin))
        .and_then(get_salary_stats);

    // kategorileri çekebileceğim bir endpoint tanımı.
    // Sadece user rolündekiler erişebilirler
    let categories_route = warp::path("categories")
        .and(warp::get())
        .and(with_auth(Role::User))
        .and_then(get_categories);

    // route tanımlamaları çalışma zamanına eklenir.
    // CORS ayarına göre herkes erişebilir
    let routes = root
        .or(login_route)
        .or(register_route)
        .or(stats_route)
        .or(categories_route)
        .with(warp::cors().allow_any_origin())
        .recover(catch_rejection);

    info!("Sunucu dinlemede. http:://127.0.0.1:5555");
    // 127.0.0.1:5555 porttan sunucu açılır
    warp::serve(routes).run(([127, 0, 0, 1], 5555)).await;
}
