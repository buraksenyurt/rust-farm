mod map;

// module erişimlerini kolaylaştırmak için prelude isimli bir başla module tanımladık.
// prelude isimli module zaten root module içinde yer aldığından public tanımlanmasına gerek yoktur.
mod prelude {
    // prelude'u kullanan herkes ayrıca bracket_lib::prelude modülü içinde
    // use ile tanımlanmış modüllere de erişebilecek. Pek tabii constant'lara ve
    // map modulülün tüm public enstrümanlarına...
    pub use bracket_lib::prelude::*;
    pub const SCHENE_WIDTH: i32 = 80;
    pub const SCHENE_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

fn main() {
    println!("Hello, world!");
}
