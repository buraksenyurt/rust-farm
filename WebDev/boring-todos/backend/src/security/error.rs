use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum TokenError {
    #[error("Geçersiz token {0}")]
    InvalidToken(String),
}
