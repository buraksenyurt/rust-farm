use crate::constant::SUBMARINE_DIMENSION;
use crate::level::Level;
use bracket_lib::prelude::{BTerm, BLACK, GOLD, PURPLE, TURQUOISE};

// Denizlatı ya da oyuncunun kendisi Player isimli veri yapısında tutulabilir
// x, y koordinatları ile hız bilgisini tutmaktayız
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub frame: usize,
    pub level: Level,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            frame: 0,
            level: Level::Easy,
        }
    }

    // Render fonksiyonu, BTerm nesnesinden yararlanarak
    // denizaltıyı semobilze eden bir karakteri Codepage 437'e uygun olacak şekilde
    // ekrana çizer.
    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.draw_box(
            self.x,
            self.y,
            SUBMARINE_DIMENSION[0],
            SUBMARINE_DIMENSION[1],
            PURPLE,
            GOLD,
        );
        ctx.print_color_centered(
            1,
            BLACK,
            TURQUOISE,
            &format!(
                "{}:{} Level {}({})",
                self.x,
                self.y,
                self.level,
                i32::from(&self.level)
            ),
        );
    }
}
