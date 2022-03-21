use bracket_lib::prelude::{main_loop, BError, BTermBuilder};
use state::State;

mod constant;
mod game_mode;
mod level;
mod player;
mod rock;
mod state;
/*
   bracket-lib nesne üretimlerinde builder pattern kullanır.
   Oyun sahasını başlatırken olası bir hata BError olarak çıkar.
   main bu nedenle BError dönecek şekilde değiştirilmiştir.
*/
fn main() -> BError {
    // 80X50 boyutlarında, başlığında Submarine - Red Squad yazan bir context inşa ediliyor
    let context = BTermBuilder::simple80x50()
        .with_title("Submarine - Red Squad")
        .build()?;

    // oyun motorunu, oyunun state nesnesi ile bağlıyoruz
    main_loop(context, State::new())
}
