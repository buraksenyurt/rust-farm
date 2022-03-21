use crate::constant::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::player::Player;
use bracket_lib::prelude::{BTerm, RandomNumberGenerator, BLACK, RED};

pub struct Rock {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rock {
    pub fn new() -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x: SCREEN_WIDTH,
            y: random.range(0, SCREEN_HEIGHT - 5),
            width: random.range(0, 10),
            height: random.range(0, 10),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.draw_box(self.x, self.y, self.width, self.height, RED, BLACK);
    }

    pub fn hit_rock(&self, player: &Player) -> bool {
        player.x == self.x && (player.y > self.y - 5 && player.y < self.y + 5)
    }

    pub fn forward(&mut self, x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            width: 2,
            height: 2,
        }
    }
}
