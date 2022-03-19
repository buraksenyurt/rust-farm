use crate::contant::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::player::Player;
use bracket_lib::prelude::VirtualKeyCode::B;
use bracket_lib::prelude::{to_cp437, BTerm, RandomNumberGenerator, BLACK, RED};

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
            x: random.range(SCREEN_WIDTH - 20, SCREEN_WIDTH),
            y: random.range(SCREEN_HEIGHT - 20, SCREEN_HEIGHT),
            width: random.range(1, 5),
            height: random.range(1, 5),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.draw_box(self.x, self.y, self.width, self.height, RED, BLACK);
    }

    pub fn hit_rock(&self, player: &Player) -> bool {
        player.x == self.x
    }
}
