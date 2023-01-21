use crate::data::db::UsersDb;
use crate::data::security::create_hashed_pwd;
use crate::model::user::User;
use crate::model::user_dao::UserDao;
use log::{error, info};
use warp::{
    http::{Response, StatusCode},
    Rejection, Reply,
};

// Kullanıcı oluşturma işini üstlenen fonksiyon.
pub async fn create_user(user: UserDao, db: UsersDb) -> Result<impl Reply, Rejection> {
    info!("Gelen kullanıcı verisi {:?}", user);
    // veritabanını kullanım için güvenli bir şekilde kilitliyoruz
    let mut users_db = db.lock().await;

    // Eğer kullanıcı adı hashmap'te var olan bir key ise geriye HTTP 400 Bad Request dönüyoruz
    if users_db.contains_key(&user.username) {
        error!("Bu kullanıcı zaten kayıtlı");
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Kullanıcı zaten kayıtlı.".to_string()));
    }

    // HashMap uzunluğu kullanılıp yeni bir id yakalınır.
    let last_index = users_db.keys().len();
    // User veri nesnesi örneklenir
    let registered_user = User {
        id: last_index,
        username: user.username,
        password: create_hashed_pwd(&user.password),
        role: user.role,
    };
    // sahte veritabanına kullanıcı bilgisi eklenir
    users_db.insert(registered_user.username.clone(), registered_user.clone());
    info!("Kullanıcı veritabanına eklendi {:?}", &registered_user);

    // Kayıt edilen kullanıcı bilgisi HTTP 200 Ok ile json formatında geriye döndürülür.
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&registered_user).unwrap()))
}
