use crate::prelude::*;
mod adventurer;
mod map;
mod map_builder;
mod state;

// module erişimlerini kolaylaştırmak için prelude isimli bir başla module tanımladık.
// prelude isimli module zaten root module içinde yer aldığından public tanımlanmasına gerek yoktur.
mod prelude {
    // prelude'u kullanan herkes ayrıca bracket_lib::prelude modülü içinde
    // use ile tanımlanmış modüllere de erişebilecek. Pek tabii constant'lara da
    pub use bracket_lib::prelude::*;
    pub const SCHENE_WIDTH: i32 = 80;
    pub const SCHENE_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCHENE_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCHENE_HEIGHT / 2;
    pub const FONT_SOURCE: &str = "gamefonts.png";
    pub use crate::adventurer::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::state::*;
}

// Ana fonksiyon olası panik durumuna göre BError döndürmekte
fn main() -> BError {
    // context'i inşa ediyoruz.
    // 80X50 pixel boyutlarında bir saha.
    // Başlığında Dungeon Crawler yazıyor ve saniye 30 çerçevelik bir oyun hızı var.
    // Oyun karakteri, yollar ve duvarlar için resource klasöründe yer alan png kullanılıyor.
    // Bu png üstünde 32X32 pixel boyutlarında ASCII glpyh'leri mevcut.
    // render fonksiyonlarında kullanılan sembollerin bu png'de karşılık geldiği figürü,
    // bracket-lib otomatik olarak ele alabiliyor. Tabii bunun için tile_dimension değerini,
    // resource'ların nerene olduğunu belirtmek lazım.
    // Ayrıca oyunun bu sürümünde iki katman(Layer) kullanılmakta.
    // Birisi zemine diğeri oyuncu karakterine ait.
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font(FONT_SOURCE, 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_SOURCE)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_SOURCE)
        .build()?;

    // Oyun döngüsü başlatılıyor ve yeni bir State nesnesi ile ilişkilendiriliyor
    main_loop(context, State::new())
}
