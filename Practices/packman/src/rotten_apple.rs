use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct RottenApple {
    pub location: Point,
    is_eat: bool,
}

impl RottenApple {
    pub fn new(location: Point) -> Self {
        Self {
            location,
            is_eat: false,
        }
    }

    pub fn is_eated(&self) -> bool {
        self.is_eat
    }

    pub fn eated(&mut self) {
        self.is_eat = true;
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.set(
            self.location.x,
            self.location.y,
            WHITE,
            YELLOW,
            to_cp437(','),
        )
    }
}
