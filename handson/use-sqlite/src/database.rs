use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
use std::env;

pub fn open_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL bilgisi eksik");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("{} veritabanına bağlanılamadı", database_url))
}
