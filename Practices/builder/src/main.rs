use crate::game::Game;

mod game;

fn main() {
    // #1 Bir nesneyi oluşturmanın basit yolu
    let game_1 = Game {
        title: "Prince of Persia".to_string(),
        released: false,
        summary: None,
    };
    println!("{game_1:#?}");

    // #2 bir constructor fonksiyon yardımıyla nesne oluşturmak
    let game_2 = Game::new("Command & Conquer Red Alert II");
    println!("{game_2:#?}");

    // #3 Bir başka nesne oluşturma yolu da Default trait'den implemente etmektir.
}
