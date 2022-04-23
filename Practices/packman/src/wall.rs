use super::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Wall {
    pub location: Point,
    is_blasted: bool,
}

impl Wall {
    pub fn new(location: Point) -> Self {
        Self {
            location,
            is_blasted: false,
        }
    }

    pub fn blasted(&mut self) {
        self.is_blasted = true;
    }

    pub fn is_blasted(&self) -> bool {
        self.is_blasted
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.set(
            self.location.x,
            self.location.y,
            WHITE,
            BLACK,
            to_cp437('!'),
        )
    }
}
