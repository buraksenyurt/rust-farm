use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Apple {
    pub location: Point,
}

impl Apple {
    pub fn new(location: Point) -> Self {
        Self { location }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.set(self.location.x, self.location.y, RED, WHITE, to_cp437('.'))
    }
}
