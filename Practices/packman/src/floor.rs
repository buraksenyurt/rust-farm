use super::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Floor {
    pub location: Point,
}

impl Floor {
    pub fn new(location: Point) -> Self {
        Self { location }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.set(self.location.x, self.location.y, BLACK, RED, to_cp437('#'))
    }
}
