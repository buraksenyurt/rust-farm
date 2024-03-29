use crate::router::{del_game, get_categories, get_games, post_game, put_game};
use actix_web::web::{delete, get, post, put};
use actix_web::{App, HttpServer};

mod database;
mod model;
mod router;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Sunucu 127.0.0.1:5232 adresine açılıyor");
    HttpServer::new(|| {
        App::new()
            .route("/categories", get().to(get_categories))
            .route("/games", post().to(post_game))
            .route("/games", get().to(get_games))
            .route("/games/{name}", delete().to(del_game))
            .route("/games", put().to(put_game))
    })
    .bind(("127.0.0.1", 5232))?
    .run()
    .await
}

// fn main() {
//     let conn = &mut open_connection();
//     let menu = "Ne yapmak istersin?
//             0 - Kategorileri göster
//             1 - Yeni bir oyun ekle
//             2 - Oyunları göster
//             3 - Oyun sil
//             4 - Oyun bilgilerini güncelle
//             5 - Menüyü göster
//             6 - Programdan çık
//         ";
//     println!("{}", menu.blue());
//
//     loop {
//         let mut choise = String::new();
//         stdin().read_line(&mut choise).unwrap();
//         let choise = choise.trim_end();
//         match choise {
//             "0" => {
//                 let category_list = categories
//                     .load::<Category>(conn)
//                     .expect("Kategoriler yüklenemedi");
//
//                 for c in category_list {
//                     println!("{}", c.to_string().yellow());
//                 }
//             }
//             "1" => {
//                 let mut title = String::new();
//                 let mut stars = String::new();
//                 let mut cat_id = String::new();
//
//                 println!("Oyunun adı ?");
//                 stdin().read_line(&mut title).unwrap();
//                 let title = title.trim_end();
//
//                 println!("Kategori numarası ?");
//                 stdin().read_line(&mut cat_id).unwrap();
//                 let cat_id = cat_id.trim_end();
//
//                 println!("Puanı ?");
//                 stdin().read_line(&mut stars).unwrap();
//                 let stars = stars.trim_end();
//
//                 let new_game = UpsertGame::new(
//                     i32::from_str(cat_id).expect("Geçersiz sayısal değer"),
//                     title,
//                     i32::from_str(stars).expect("Geçersiz sayısal değer"),
//                 );
//
//                 let inserted = create_game(conn, new_game);
//                 println!("{} Oyun yüklendi", inserted.to_string().green());
//             }
//             "2" => {
//                 let category_list = categories
//                     .load::<Category>(conn)
//                     .expect("Kategoriler yüklenemedi");
//
//                 for c in category_list {
//                     println!("{}", c.to_string().yellow().bold());
//                     let game_list = games
//                         .filter(category_id.eq(c.id))
//                         .load::<Game>(conn)
//                         .expect("Oyunlar listelenemiyor");
//
//                     for g in game_list {
//                         println!("\t{}", g.to_string().blue());
//                     }
//                 }
//             }
//             "3" => {
//                 let mut game_name = String::new();
//
//                 println!("Oyun Adı ?");
//                 stdin().read_line(&mut game_name).unwrap();
//                 let game_name = game_name.trim_end();
//                 let pattern = format!("%{}%", game_name);
//                 let deleted = delete_game(conn, pattern);
//                 println!("Adında '{}' geçen {} oyun silindi.", game_name, deleted);
//             }
//             "4" => {
//                 let mut game_id = String::new();
//                 println!("Oyun numarası ?");
//                 stdin().read_line(&mut game_id).unwrap();
//                 let game_id = game_id.trim_end();
//                 let id = i32::from_str(game_id).expect("ID değeri sayısal olmalı");
//
//                 let mut title = String::new();
//                 let mut stars = String::new();
//                 let mut cat_id = String::new();
//
//                 println!("Oyunun adı ?");
//                 stdin().read_line(&mut title).unwrap();
//                 let title = title.trim_end();
//
//                 println!("Kategori numarası ?");
//                 stdin().read_line(&mut cat_id).unwrap();
//                 let cat_id = cat_id.trim_end();
//
//                 println!("Puanı ?");
//                 stdin().read_line(&mut stars).unwrap();
//                 let stars = stars.trim_end();
//
//                 let updated_game = UpsertGame::new(
//                     i32::from_str(cat_id).expect("Geçersiz sayısal değer"),
//                     title,
//                     i32::from_str(stars).expect("Geçersiz sayısal değer"),
//                 );
//
//                 let updated_count = update_game(conn, id, updated_game);
//                 println!("{} kayıt güncellendi", updated_count.to_string().green());
//             }
//             "5" => {
//                 println!("{}", menu.blue());
//             }
//             "6" => exit(0),
//             _ => {
//                 println!("Seçimini anlayamadım");
//             }
//         }
//     }
// }
