use crate::model::NewGame;
use crate::schema::games;
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use dotenvy::dotenv;
use std::env;

pub fn open_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL bilgisi eksik");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("{} veritabanına bağlanılamadı", database_url))
}

pub fn create_game(conn: &mut SqliteConnection, new_game: NewGame) -> usize {
    diesel::insert_into(games::table)
        .values(&new_game)
        .execute(conn)
        .expect("Yeni oyun kaydedilirken hata")
}
