use crate::data::db::UsersDb;
use crate::error::custom_error::CustomError;
use crate::error::handler::Result;
use crate::model::login_user::LoginUser;
use crate::model::user::User;
use crate::model::user_dao::UserDao;
use crate::security::auditer::{create_hashed_pwd, create_jwt, verify_pwd};
use log::{error, info};
use warp::{
    http::{Response, StatusCode},
    reject, Reply,
};

// Kullanıcı oluşturma işini üstlenen fonksiyon.
pub async fn create_user(user: UserDao, db: UsersDb) -> Result<impl Reply> {
    info!("Gelen kullanıcı verisi {:?}", user);
    // veritabanını kullanım için güvenli bir şekilde kilitliyoruz
    let mut users_db = db.lock().await;

    // Eğer kullanıcı adı hashmap'te var olan bir key ise geriye HTTP 400 Bad Request dönüyoruz
    if users_db.contains_key(&user.username) {
        error!("Bu kullanıcı zaten kayıtlı");
        // return Ok(Response::builder()
        //     .status(StatusCode::BAD_REQUEST)
        //     .body("Kullanıcı zaten kayıtlı.".to_string()));
        return Err(reject::custom(CustomError::UserExists(user.username)));
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

// Login işleminin gerçekleştirildiği handler fonksiyonu
pub async fn login(login_user: LoginUser, db: UsersDb) -> Result<impl Reply> {
    // Kullanmak üzere db nesnesi için thread safe bir kilitleme yapılır
    let users_db = db.lock().await;

    // login olunmak istenen kullanıcı sahte veritabanımızda var mı yok mu bir bakıyoruz.
    let user = match users_db.get(&login_user.username) {
        Some(u) => u,
        None => {
            error!("{} isimli kullanıcı bulunamadı", &login_user.username);
            return Err(reject::custom(CustomError::InvalidCredentials));
            // return Ok(Response::builder()
            //     .status(StatusCode::BAD_REQUEST)
            //     .body("Kullanıcı bulunamadı".to_string()));
        }
    };

    info!("Kullanıcı bulundu. Şifre kontrol ediliyor...");
    // kullanıcının girdiği şifre veritabanına hashlenerek kaydedilen ile karşılaştırılıyor.
    // eğer hatalı şifre ise Bad Request ile cezalandırılıyor.
    if !verify_pwd(&login_user.password, &user.password) {
        error!("{} için hatalı şifre", &login_user.username);
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Login hatası".to_string()));
    }

    info!("Giriş başarılı");
    // HTTP 200 Ok
    // Kullanıcı ve şifre geçerli. Bu durumda JWT token bilgisi üretip geriye dönüyoruz.
    let token = create_jwt(user);
    info!("Üretilen token {}", token);
    Ok(Response::builder().status(StatusCode::OK).body(token))
}
