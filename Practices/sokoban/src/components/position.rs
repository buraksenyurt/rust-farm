use crate::prelude::*;

#[derive(Debug, Component, Copy, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}
