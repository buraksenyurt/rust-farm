use mobc_postgres::tokio_postgres;
use thiserror::Error;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Havuzdan bağlantı alma hatası {0}")]
    ConnPoolError(mobc::Error<tokio_postgres::Error>),
    #[error("Veritabanı başlatma hatası")]
    DbInitError(tokio_postgres::Error),
    #[error("Sorgu işletme hatası")]
    DbQueryError(tokio_postgres::Error),
}

impl Reject for DbError {}
