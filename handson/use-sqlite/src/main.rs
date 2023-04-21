use crate::database::{create_game, open_connection};
use crate::model::{Category, Game, NewGame};
use crate::schema::categories::dsl::categories;
use crate::schema::games::category_id;
use crate::schema::games::dsl::games;
use colored::Colorize;
use diesel::prelude::*;
use std::io::stdin;
use std::process::exit;
use std::str::FromStr;

mod database;
mod model;
mod schema;

fn main() {
    let conn = &mut open_connection();
    let menu = "Ne yapmak istersin?
            0 - Kategorileri göster
            1 - Yeni bir oyun ekle
            2 - Oyunları göster
            3 - Programdan çık
        ";
    loop {
        println!("{}", menu.blue());

        let mut choise = String::new();
        stdin().read_line(&mut choise).unwrap();
        let choise = choise.trim_end();
        match choise {
            "0" => {
                let category_list = categories
                    .load::<Category>(conn)
                    .expect("Kategoriler yüklenemedi");

                for c in category_list {
                    println!("{} - {}", c.id, c.title);
                }
            }
            "1" => {
                let mut title = String::new();
                let mut stars = String::new();
                let mut cat_id = String::new();

                println!("Oyunun adı ?");
                stdin().read_line(&mut title).unwrap();
                let title = title.trim_end();

                println!("Kategori numarası ?");
                stdin().read_line(&mut cat_id).unwrap();
                let cat_id = cat_id.trim_end();

                println!("Puanı ?");
                stdin().read_line(&mut stars).unwrap();
                let stars = stars.trim_end();

                create_game(
                    conn,
                    NewGame {
                        title,
                        category_id: i32::from_str(cat_id).expect("Geçersiz sayısal değer"),
                        stars: i32::from_str(stars).expect("Geçersiz sayısal değer"),
                    },
                );
                println!("Oyun yüklendi");
            }
            "2" => {
                let category_list = categories
                    .load::<Category>(conn)
                    .expect("Kategoriler yüklenemedi");

                for c in category_list {
                    println!("{} - {}", c.id, c.title);
                    let game_list = games
                        .filter(category_id.eq(c.id))
                        .load::<Game>(conn)
                        .expect("Oyunlar listelenemiyor");

                    for g in game_list {
                        println!("\t{} - {} Start({})", g.id, g.title, g.stars);
                    }
                }
            }
            "3" => exit(0),
            _ => {
                println!("Seçimini anlayamadım");
            }
        }
    }
}
