use crate::prelude::*;

const NUMBER_OF_TILES: usize = { SCHENE_WIDTH * SCHENE_HEIGHT } as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum ObjectType {
    Wall,
    Floor,
    Apple,
    RottenApple,
}
pub struct Map {
    pub objects: Vec<ObjectType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            objects: vec![ObjectType::Floor; NUMBER_OF_TILES],
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCHENE_HEIGHT {
            for x in 0..SCHENE_WIDTH {
                let index = map_to_index(x, y);
                match self.objects[index] {
                    ObjectType::Wall => ctx.set(x, y, WHITE, BLACK, to_cp437('#')),
                    ObjectType::Floor => ctx.set(x, y, WHITE, BLACK, to_cp437('!')),
                    ObjectType::Apple => ctx.set(x, y, WHITE, BLACK, to_cp437('.')),
                    ObjectType::RottenApple => ctx.set(x, y, RED, BLACK, to_cp437(',')),
                }
            }
        }
    }
    pub fn is_in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCHENE_WIDTH && point.y >= 0 && point.y < SCHENE_HEIGHT
    }
    pub fn want_enter_tile(&self, point: Point) -> bool {
        self.is_in_bounds(point)
            && self.objects[map_to_index(point.x, point.y)] == ObjectType::Floor
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
    ((y + SCHENE_WIDTH) + x) as usize
}
