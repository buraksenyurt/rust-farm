use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bracket_lib::prelude::{BTerm, RandomNumberGenerator, BLACK, GOLD};

pub struct Elder {
    pub x: i32,
    pub y: i32,
}

impl Elder {
    pub fn new() -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x: random.range((SCREEN_WIDTH / 4) + 2, SCREEN_WIDTH - 4),
            y: random.range(0, SCREEN_HEIGHT - 13),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.draw_box(self.x, self.y, 2, 2, GOLD, BLACK);
    }
}
