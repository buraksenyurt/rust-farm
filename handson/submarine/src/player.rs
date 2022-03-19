use crate::contant::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bracket_lib::prelude::{to_cp437, BTerm, BLACK, GOLD, PURPLE, TURQUOISE};

// Denizlatı ya da oyuncunun kendisi Player isimli veri yapısında tutulabilir
// x, y koordinatları ile hız bilgisini tutmaktayız
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub speed: f32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y, speed: 0.0 }
    }

    // Render fonksiyonu, BTerm nesnesinden yararlanarak
    // denizaltıyı semobilze eden bir karakteri Codepage 437'e uygun olacak şekilde
    // ekrana çizer.
    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, PURPLE, GOLD, to_cp437('>'));
        ctx.print_color_centered(1, BLACK, TURQUOISE, self.y);
    }

    pub fn up(&mut self) {
        self.y -= 1;
        if self.y <= 2 {
            self.y = 2;
        }
    }

    pub fn down(&mut self) {
        self.y += 1;
        if self.y >= SCREEN_HEIGHT {
            self.y = SCREEN_HEIGHT - 1;
        }
    }
}
