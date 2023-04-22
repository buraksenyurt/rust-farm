use crate::model::{Category, Game, UpsertGame};
use crate::schema::categories::dsl::categories;
use crate::schema::games::dsl::games;
use crate::schema::games::{category_id, stars, title};
use diesel::{
    Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection, TextExpressionMethods,
};
use dotenvy::dotenv;
use std::env;

pub fn open_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL bilgisi eksik");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("{} veritabanına bağlanılamadı", database_url))
}

pub fn load_categories(conn: &mut SqliteConnection) -> Vec<Category> {
    categories
        .load::<Category>(conn)
        .expect("Kategoriler yüklenemedi")
}

pub fn load_games(conn: &mut SqliteConnection) -> Vec<Game> {
    games.load::<Game>(conn).expect("Oyunlar yüklenemedi")
}

pub fn create_game(conn: &mut SqliteConnection, new_game: UpsertGame) -> usize {
    println!(
        "{},{},{} eklenecek.",
        new_game.title, new_game.stars, new_game.category_id
    );
    diesel::insert_into(games)
        .values(&new_game)
        .execute(conn)
        .expect("Yeni oyun kaydedilirken hata")
}

pub fn update_game(conn: &mut SqliteConnection, id: i32, updated: UpsertGame) -> usize {
    diesel::update(games.find(id))
        .set((
            title.eq(updated.title),
            stars.eq(updated.stars),
            category_id.eq(updated.category_id),
        ))
        .execute(conn)
        .expect("Güncelleme işleminde hata")
}

pub fn delete_game(conn: &mut SqliteConnection, pattern: &str) -> usize {
    diesel::delete(games.filter(title.like(pattern)))
        .execute(conn)
        .expect("Oyun silinirken hata oluştu")
}
