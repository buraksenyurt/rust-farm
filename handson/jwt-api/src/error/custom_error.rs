use thiserror::Error;
use warp::reject::Reject;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("kullanıcı zaten var")]
    UserExists(String),
    #[error("ehliyet yok")]
    InvalidCredentials,
    #[error("Geçersiz başlık bilgisi")]
    AutoHeaderRequired,
    #[error("Geçersiz bilet")]
    InvalidToken,
    #[error("JWT token üretim hatası")]
    TokenCreation,
    #[error("Eksik yetki")]
    NotAuthorized,
}

impl Reject for CustomError {}
