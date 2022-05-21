use bonsaidb::{
    core::schema::{Collection, SerializedCollection},
    local::{
        config::{Builder, StorageConfiguration},
        Database,
    },
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/*
   BonsaiDb doküman tabanlı bir veritabanı sistemi.
   Her bir satır bir document olarak tutuluyor.
   Elbette, dokümanın benzersiz bir IDsi oluyor.
   Serileşebilir bir veri yapısı olması lazım.
*/
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "games")]
struct Game {
    pub timestamp: SystemTime,
    pub name: String,
    pub max_player_count: i32,
}

impl Game {
    pub fn new(name: String, max_player_count: i32) -> Self {
        Self {
            name,
            max_player_count,
            timestamp: SystemTime::now(),
        }
    }
}

fn main() -> Result<(), bonsaidb::core::Error> {
    // Game veri yapısını kullanan worlddb isimli bir veritabanı oluşturuyoruz
    let db = Database::open::<Game>(StorageConfiguration::new("worlddb"))?;

    // Örnek bir veri satırı(doküman) oluşturduk ve veritabanına ekledik
    let wwc = Game::new("Vörl ov vorkıraft".to_string(), 8).push_into(&db)?;

    let sf = Game {
        timestamp: SystemTime::now(),
        name: "Sitrit Faytaa".to_string(),
        max_player_count: 16,
    }
    .push_into(&db)?;

    // Eklenen dokümanları db'den çekmeyi deniyoruz
    let game = Game::get(&wwc.header.id, &db)?.expect("İlgili öğe bulunamadı");
    println!("Id: {}\n{:#?}", game.header.id, game);

    let game = Game::get(&sf.header.id, &db)?.expect("İlgili öğe bulunamadı");
    println!("Id: {}\n{:#?}", game.header.id, game);

    Ok(())
}
