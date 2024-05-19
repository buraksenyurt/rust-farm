use rusqlite::Error as RusqliteError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("database error: {0}")]
    DatabaseError(#[from] RusqliteError),
    #[error("item not found")]
    NotFound,
}
