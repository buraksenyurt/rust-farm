use crate::prelude::*;
mod adventurer;
mod map;
mod state;

// module erişimlerini kolaylaştırmak için prelude isimli bir başla module tanımladık.
// prelude isimli module zaten root module içinde yer aldığından public tanımlanmasına gerek yoktur.
mod prelude {
    // prelude'u kullanan herkes ayrıca bracket_lib::prelude modülü içinde
    // use ile tanımlanmış modüllere de erişebilecek. Pek tabii constant'lara ve
    // map modulülün tüm public enstrümanlarına...
    pub use bracket_lib::prelude::*;
    pub const SCHENE_WIDTH: i32 = 80;
    pub const SCHENE_HEIGHT: i32 = 50;
    pub use crate::adventurer::*;
    pub use crate::map::*;
    pub use crate::state::*;
}

// Ana fonksiyon olası panik durumuna göre BError döndürmekte
fn main() -> BError {
    // context'i inşa ediyoruz.
    // 80X50 pixel boyutlarında bir saha.
    // Başlığında Dungeon Crawler yazıyor ve saniye 30 çerçevelik bir oyun hızı var.
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;

    // Oyun döngüsü başlatılıyor ve yeni bir State nesnesi ile ilişkilendiriliyor
    main_loop(context, State::new())
}
