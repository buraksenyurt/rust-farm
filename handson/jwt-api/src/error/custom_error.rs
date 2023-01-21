use thiserror::Error;
use warp::reject::Reject;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("kullanıcı zaten var")]
    UserExists(String),
    #[error("yetki yok")]
    InvalidCredentials,
}

impl Reject for CustomError {}
