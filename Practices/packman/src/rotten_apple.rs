use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct RottenApple {
    pub location: Point,
}

impl RottenApple {
    pub fn new(location: Point) -> Self {
        Self { location }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.set(self.location.x, self.location.y, WHITE, YELLOW, to_cp437(','))
    }
}
