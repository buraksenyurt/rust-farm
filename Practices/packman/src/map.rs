use crate::prelude::*;

const NUMBER_OF_TILES: usize = { DISPLAY_WIDTH * DISPLAY_HEIGHT } as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum ObjectType {
    Floor,
    Wall,
    Apple(usize),
    RottenApple,
}
pub struct Map {
    pub objects: Vec<ObjectType>,
    pub tiles: Vec<Tile>,
    pub walls: Vec<Wall>,
    pub apples: Vec<Apple>,
    pub roten_apples: Vec<RottenApple>,
    pub player_score: i32,
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
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        //info!("TOTAL TILE {}", self.objects.len());

        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                // let index = map_to_index(x, y);
                // match self.objects[index] {
                //     ObjectType::Floor => ctx.set(x, y, BLACK, RED, to_cp437('#')),
                //     ObjectType::Wall => ctx.set(x, y, WHITE, BLACK, to_cp437('!')),
                //     ObjectType::Apple => ctx.set(x, y, WHITE, RED, to_cp437('.')),
                //     ObjectType::RottenApple => ctx.set(x, y, WHITE, YELLOW, to_cp437(',')),
                // }
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
            rotten_apple.render(ctx);
        }

        ctx.set_active_console(4);
        for wall in &self.walls {
            wall.render(ctx);
        }

        ctx.set_active_console(5);
        ctx.cls();
        ctx.print_centered_at(DISPLAY_WIDTH / 2, 0, format!("SKOR:{}", self.player_score));
    }
    pub fn is_in_bounds(&self, point: Point) -> bool {
        (point.x >= 0 && point.x < DISPLAY_WIDTH) && (point.y >= 0 && point.y < DISPLAY_HEIGHT)
    }
    pub fn try_map_to_index(&self, point: Point) -> Option<usize> {
        if !self.is_in_bounds(point) {
            None
        } else {
            Some(map_to_index(point.x, point.y))
        }
    }
}

pub fn map_to_index(x: i32, y: i32) -> usize {
    ((y * DISPLAY_WIDTH) + x) as usize
}
