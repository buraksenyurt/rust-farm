use crate::prelude::*;

pub struct Boss {
    pub location: Point,
}

impl Boss {
    pub fn new(location: Point) -> Self {
        Self { location }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.set(
            self.location.x,
            self.location.y,
            YELLOW,
            WHITE,
            to_cp437('X'),
        )
    }
}
