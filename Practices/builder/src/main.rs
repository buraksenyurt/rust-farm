use crate::common::*;
use crate::game::Game;
use crate::web_request::WebRequestBuilder;

mod common;
mod game;
mod web_request;

fn main() -> Result<()> {
    /*
       1,2,3 ve 4 numaralı örnekler basit alanlar içeren kolak oluşturulabilir nesneler içindir.
       Daha karmaşık nesnelerin oluşturulmasında builder pattern'lerden yararlanılabilir.
    */
    // #1 Bir nesneyi oluşturmanın basit yolu
    let game = Game {
        title: "Prince of Persia".to_string(),
        released: false,
        summary: None,
    };
    println!("{game:#?}");

    // #2 bir constructor fonksiyon yardımıyla nesne oluşturmak
    let game = Game::new("Command & Conquer Red Alert II");
    println!("{game:#?}");

    // #3 Bir başka nesne oluşturma yolu da Default trait'den implemente etmektir.
    let game = Game::default();
    println!("{game:#?}");
    // Pek çok fonksiyon Default trait implementasyonuna bakar. Örneğin,
    let game: Option<Game> = None; // None olarak gelen bir Game değişkeni
    let game = game.unwrap_or_default(); // None olduğu için Default fonksiyonu çağrılır
    println!("{game:#?}");

    // Default trait'i aşağıdaki gibi kullanımları da mümkün kılar
    let game = Game {
        released: true,
        ..Default::default()
    };
    println!("{game:#?}");

    // #5 Bir nesneyi builder fonksiyonu ile oluşturmak
    let request = WebRequestBuilder::new()
        .url("http://localhost:5679/api/games")
        .method("POST")
        .header("content-type", "application/json")
        .body("{'name':'Barbarian'}")
        .build()?;
    println!("{request:#?}");

    Ok(())
}
