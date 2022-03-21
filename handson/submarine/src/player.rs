use crate::constant::{SCREEN_HEIGHT, SCREEN_WIDTH, SUBMARINE_DIMENSION};
use crate::level::Level;
use bracket_lib::prelude::{BTerm, BLACK, GOLD, PURPLE, TURQUOISE};

// Denizlatı ya da oyuncunun kendisi Player isimli veri yapısında tutulabilir
// x, y koordinatları ile hız bilgisini tutmaktayız
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub speed: f32,
    pub frame: usize,
    pub level: Level,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            speed: 0.0,
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
        ctx.print_color_centered(1, BLACK, TURQUOISE, &format!("{}:{}", self.x, self.y));
    }

    pub fn up(&mut self) {
        self.y -= i32::from(&self.level);
        if self.y <= 2 {
            self.y = 2;
        }
    }

    pub fn down(&mut self) {
        self.y += i32::from(&self.level);
        if self.y >= SCREEN_HEIGHT {
            self.y = SCREEN_HEIGHT - 1;
        }
    }

    pub fn right(&mut self) {
        self.x += i32::from(&self.level);
        if self.x >= SCREEN_WIDTH {
            self.x = SCREEN_WIDTH - 5
        }
    }

    pub fn left(&mut self) {
        self.x -= i32::from(&self.level);
        if self.x <= 2 {
            self.x = 2;
        }
    }
}
