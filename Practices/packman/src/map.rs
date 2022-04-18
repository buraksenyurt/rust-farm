use crate::prelude::*;

const NUMBER_OF_TILES: usize = { SCHENE_WIDTH * SCHENE_HEIGHT } as usize;

#[derive(Copy, Clone)]
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
}
