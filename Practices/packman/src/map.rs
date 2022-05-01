use crate::prelude::*;
use std::time::Duration;

const NUMBER_OF_TILES: usize = { DISPLAY_WIDTH * DISPLAY_HEIGHT } as usize;

pub struct Map {
    pub objects: Vec<ObjectType>,
    pub tiles: Vec<Tile>,
    pub walls: Vec<Wall>,
    pub apples: Vec<Apple>,
    pub roten_apples: Vec<Apple>,
    pub player_score: i32,
    pub bomb_count: u16,
    pub warp_level: u16,
    pub start_time: Instant,
    pub duration: Duration,
    pub eated_apple_count: usize,
}

impl Map {
    pub fn new() -> Self {
        Self {
            objects: vec![ObjectType::Floor; NUMBER_OF_TILES],
            tiles: Vec::new(),
            apples: Vec::new(),
            roten_apples: Vec::new(),
            walls: Vec::new(),
            player_score: 0,
            bomb_count: BOMB_COUNT,
            warp_level: WARP_MAX_LEVEL,
            start_time: Instant::now(),
            duration: Duration::ZERO,
            eated_apple_count: 0,
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(0);

        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let floor = Floor::new(Point { x, y });
                floor.render(ctx);
            }
        }

        ctx.set_active_console(2);
        for apple in &self.apples {
            if apple.is_eated() {
                continue;
            }
            apple.render(ctx);
        }

        ctx.set_active_console(3);
        for rotten_apple in &self.roten_apples {
            if rotten_apple.is_eated() {
                continue;
            }
            rotten_apple.render(ctx);
        }

        ctx.set_active_console(4);
        for wall in &self.walls {
            if wall.is_blasted() {
                continue;
            }
            wall.render(ctx);
        }

        ctx.set_active_console(5);
        ctx.cls();
        ctx.print_centered_at(
            DISPLAY_WIDTH / 2,
            0,
            format!(
                "S:{} B:{} W:%{} E:{}",
                self.player_score, self.bomb_count, self.warp_level, self.eated_apple_count
            ),
        );
    }
}
