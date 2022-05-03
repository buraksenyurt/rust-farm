use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::elder::Elder;
use bracket_lib::prelude::{
    BTerm, GameState, VirtualKeyCode, BLACK, BLUE2, GOLD, GRAY, GREEN, PINK, PINK2, WHITE,
};

pub struct State {
    pub current_tick: u16,
}

impl State {
    pub fn new() -> Self {
        Self { current_tick: 0 }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => {
                    ctx.quitting = true;
                }
                _ => {}
            }
        }

        //ctx.draw_bar_horizontal(0, SCREEN_HEIGHT-1, SCREEN_WIDTH, 0, 0, WHITE, GOLD);
        ctx.draw_box(0, SCREEN_HEIGHT - 10, SCREEN_WIDTH - 1, 9, WHITE, BLUE2);
        ctx.print_color_centered_at(
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT - 8,
            PINK2,
            BLACK,
            "SCORE BOARD",
        );
        ctx.draw_box(0, 0, SCREEN_WIDTH / 4, SCREEN_HEIGHT - 11, GRAY, GOLD);
        ctx.print_color_centered_at(
            SCREEN_WIDTH / 8,
            (SCREEN_HEIGHT - 11) / 2,
            PINK2,
            BLACK,
            "LEFT SIDE",
        );
        ctx.draw_box(
            (SCREEN_WIDTH / 4) + 1,
            0,
            SCREEN_WIDTH - ((SCREEN_WIDTH / 4) + 2),
            SCREEN_HEIGHT - 11,
            PINK,
            GREEN,
        );
        ctx.print_color_centered_at(
            (SCREEN_WIDTH / 2) + (SCREEN_WIDTH / 8),
            (SCREEN_HEIGHT - 11) / 2,
            PINK2,
            BLACK,
            "CENTER SIDE",
        );

        let mut elder = Elder::new();
        elder.render(ctx);
    }
}
