use crate::prelude::*;

#[derive(Debug, Component, Copy, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Position {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }
}
