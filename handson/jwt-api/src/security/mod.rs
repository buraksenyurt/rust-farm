use crate::model::claim::Claim;
use crate::model::user::User;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};

// Kullanıcı şifrelerini hash değer karşılığı ile tutmak için yardımcı fonksiyon
pub fn create_hashed_pwd(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Scrypt
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_pwd(password: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash).unwrap();
    Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

// JWT Token üreten fonksiyon
pub fn create_jwt(user: &User) -> String {
    dotenv::dotenv().ok();

    // Sembolik olarak 5 dakikalık bir geçerlilik süresi tayin edilir.
    let expr_time = Utc::now()
        .checked_add_signed(Duration::minutes(5))
        .expect("Geçersiz zaman damgası")
        .timestamp();
    // Token içerisine gömülecek bazı bilgileri içeren claim nesnesi örneklenir.
    // Burada kullanıcı adı, rol ve geçerlilik süresi bilgileri yer alıyor.
    let claims = Claim {
        subject: user.username.clone(),
        role: user.role.clone(),
        expiration: expr_time as usize,
    };

    // .env uzantılı dosyadan JWT için kullanılacak gizli anahtar bilgisi alınır
    let jwt_secret = std::env::var("JWT_SECRET").unwrap().into_bytes();

    //let jwt_secret="0ldukc@Z0r1Secr4tkonuluyorBUray19!".to_string().into_bytes();

    // jsonwebtoken crate'in encode fonksiyonundan yararlanılarak JWT token bilgisi üretilir
    let jwt_token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&jwt_secret),
    ) {
        Ok(token) => token,
        Err(_) => panic!(),
    };

    jwt_token
}
