use bracket_lib::prelude::{BLACK, BLUE2, BTerm, GameState, GOLD, GRAY, GREEN, PINK, PINK2, WHITE};
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {

        //ctx.draw_bar_horizontal(0, SCREEN_HEIGHT-1, SCREEN_WIDTH, 0, 0, WHITE, GOLD);
        ctx.draw_box(0,SCREEN_HEIGHT-10,SCREEN_WIDTH-1,9,WHITE,BLUE2);
        ctx.print_color_centered_at(SCREEN_WIDTH/2,SCREEN_HEIGHT-8,PINK2,BLACK,"SCORE BOARD");
        ctx.draw_box(0,0,SCREEN_WIDTH/4,SCREEN_HEIGHT-11,GRAY,GOLD);
        ctx.draw_box((SCREEN_WIDTH/4)+1,0,SCREEN_WIDTH-((SCREEN_WIDTH/4)+2),SCREEN_HEIGHT-11,PINK,GREEN);
    }
}
