use crate::error::{custom_error::CustomError, handler::Result};
use crate::model::claim::Claim;
use crate::model::user::User;
use crate::security::role::Role;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, error, info};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
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
        sub: user.username.clone(),
        role: user.role.clone(),
        exp: expr_time as usize,
    };

    // .env uzantılı dosyadan JWT için kullanılacak gizli anahtar bilgisi alınır
    let jwt_secret = std::env::var("JWT_SECRET").unwrap().into_bytes();

    //let jwt_secret="0ldukc@Z0r1Secr4tkonuluyorBUray19!".to_string().into_bytes();

    // jsonwebtoken crate'in encode fonksiyonundan yararlanılarak JWT token bilgisi üretilir
    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&jwt_secret),
    ) {
        Ok(token) => token,
        Err(_) => panic!(),
    }
}

// Bazı route tanımlamalarında Authorization gerekiyorsa
// bu fonksiyon ile çalışma zamanına yükleyebiliriz.
pub fn with_auth(r: Role) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (r.clone(), headers))
        .and_then(authorize)
}

// Claim'den çekilen Role bilgisine göre kullanıcının yetkisinin olup olmadığını kontrol eden
// yardımcı fonksiyon
fn is_authorized(required_role: Role, claims_role: &str) -> bool {
    let claims_role = Role::from_str(claims_role);
    debug!(
        "Gerekli rol: {}, Kullanıcı rolü: {}",
        required_role, claims_role
    );
    required_role == claims_role || claims_role == Role::Admin
}

// Talebe ait Header bilgisinden JWT token'ı çeken fonksiyon
pub fn get_jwt_token_from_header(
    headers: &HeaderMap<HeaderValue>,
) -> std::result::Result<String, CustomError> {
    // Header gelen bir Authorization bölümü var mı kontrol edilir
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => {
            error!("Header'dan Authorization bölümü alınamadı");
            return Err(CustomError::AutoHeaderRequired);
        }
    };
    // Authorization Header içinde bilgi var mı buna bakılır
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => {
            error!("UTF8 dönüşümünde hata");
            return Err(CustomError::AutoHeaderRequired);
        }
    };

    // Bearer token olup olmadığı kontrol edilir.
    // Bilindiği üzere Authorization header bilgisinde
    // Bearer ile başlayan bir kısım olmalı
    if !auth_header.starts_with("Bearer ") {
        error!("Authorization header Bearer ile başlamıyor");
        return Err(CustomError::AutoHeaderRequired);
    }
    Ok(auth_header.trim_start_matches("Bearer ").to_owned())
}

// Doğrulama fonksiyonu
async fn authorize((role, headers): (Role, HeaderMap<HeaderValue>)) -> Result<String> {
    info!("Aranan rol {}", role.to_string());
    // parametre olarak gelen role ve header bilgilerini kullanır
    match get_jwt_token_from_header(&headers) {
        // Bir JWT içeriği varsa bu token değerinin geçerli olup olmadığına bakılır
        Ok(jwt) => {
            let jwt_secret = std::env::var("JWT_SECRET").unwrap().into_bytes();
            let decoded = decode::<Claim>(
                &jwt,
                &DecodingKey::from_secret(&jwt_secret),
                &Validation::default(),
            )
            .map_err(|_| reject::custom(CustomError::InvalidToken))?;

            debug!("Çözümlenen claim bilgileri: {:?}", &decoded.claims);
            // Yetki kontrolü yapılır ve duruma göre talep geri çevrilir
            if !is_authorized(role, &decoded.claims.role) {
                return Err(reject::custom(CustomError::NotAuthorized));
            }

            Ok(decoded.claims.sub)
        }
        Err(e) => Err(reject::custom(e)),
    }
}
