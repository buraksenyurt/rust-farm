use crate::data::db::{get_db_conn, ConnPool};
use crate::error::custom_error::CustomError;
use crate::error::handler::Result;
use crate::model::category::Category;
use crate::model::login_user::LoginUser;
use crate::model::registered_user::RegisteredUser;
use crate::model::user::User;
use crate::security::auditer::{create_hashed_pwd, create_jwt, verify_pwd};
use log::{error, info};
use warp::reply::html;
use warp::{
    http::{Response, StatusCode},
    reject, Reply,
};

// Kullanıcı oluşturma işini üstlenen fonksiyon.
pub async fn create_user(user: LoginUser, conn_pool: ConnPool) -> Result<impl Reply> {
    info!("Gelen kullanıcı verisi {:?}", user);
    let db = get_db_conn(&conn_pool)
        .await
        .map_err(|_| reject::custom(CustomError::InternalError))?;
    let role = user.role.unwrap();
    let q = db
        .query_one(
            "SELECT username FROM users WHERE username = $1 AND role=$2;",
            &[&user.username, &role],
        )
        .await;
    if q.is_ok() {
        let username: String = q.unwrap().get(0);
        if username == user.username {
            error!("Bu kullanıcı zaten kayıtlı");
            return Err(reject::custom(CustomError::UserExists(user.username)));
        }
    }

    let pwd = create_hashed_pwd(&user.password);
    let insert_query = "INSERT INTO users (username,password,role) VALUES($1,$2,$3);";
    db.execute(insert_query, &[&user.username, &pwd, &role])
        .await
        .map_err(|_| reject::custom(CustomError::InternalError))?;

    let registered_user = RegisteredUser {
        username: user.username,
        role,
    };
    info!("Kullanıcı veritabanına eklendi {:?}", &registered_user);

    // Kayıt edilen kullanıcı bilgisi HTTP 200 Ok ile json formatında geriye döndürülür.
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&registered_user).unwrap()))
}

// Login işleminin gerçekleştirildiği handler fonksiyonu
pub async fn login(login_user: LoginUser, conn_pool: ConnPool) -> Result<impl Reply> {
    let db = get_db_conn(&conn_pool)
        .await
        .map_err(|_| reject::custom(CustomError::InternalError))?;

    let q = db
        .query_one(
            "SELECT id,username,password,role FROM users WHERE username = $1",
            &[&login_user.username],
        )
        .await
        .map_err(|_| reject::custom(CustomError::InvalidCredentials))?;

    let user = User {
        id: q.get::<&str, i32>("id"),
        username: q.get::<&str, String>("username"),
        password: q.get::<&str, String>("password"),
        role: q.get::<&str, String>("role"),
    };
    info!("Kullanıcı bulundu. Şifre kontrol ediliyor...");
    if !verify_pwd(&login_user.password, &user.password) {
        error!("{} için hatalı şifre", &login_user.username);
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Login hatası".to_string()));
    }
    info!("Giriş başarılı");
    let token = create_jwt(&user);
    info!("Üretilen token {}", token);
    Ok(Response::builder().status(StatusCode::OK).body(token))
}

// Sadece admin yetkisinde olanların görebileceği demo HTML sayfasını üreten fonksiyon
pub async fn get_salary_stats(username: String) -> Result<impl Reply> {
    info!("This is a private zone. Only admins.");

    Ok(html(format!(
        r#"
            <html>
                <head>
                    <title>Salary Statistics</title>
                </head>
                <body>
                    <h1>Salary Statistics</h1>
                    <div>Wellcome {}</div>
                </body>
            </html>
        "#,
        &username
    )))
}

// Bu da sadece user rolündekilerin erişebileceği bir endpoint temsili olsun.
pub async fn get_categories(_username: String, conn_pool: ConnPool) -> Result<impl Reply> {
    let db = get_db_conn(&conn_pool)
        .await
        .map_err(|_| reject::custom(CustomError::InternalError))?;

    let rows = db
        .query("SELECT id,title FROM categories;", &[])
        .await
        .map_err(|_| reject::custom(CustomError::InternalError))?;
    let mut categories = Vec::new();
    for row in rows {
        let c = Category {
            id: row.get(0),
            title: row.get(1),
        };
        categories.push(c);
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&categories).unwrap()))
}
