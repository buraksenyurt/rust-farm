use crate::prelude::*;

#[derive(Debug, Component, Copy, Clone)]
#[storage(VecStorage)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8,
}
