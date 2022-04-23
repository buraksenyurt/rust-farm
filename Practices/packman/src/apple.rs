use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Apple {
    pub location: Point,
    is_eat: bool,
    pub fruit_type: FruitType,
}

impl Apple {
    pub fn new(location: Point, fruit_type: FruitType) -> Self {
        Self {
            location,
            is_eat: false,
            fruit_type,
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
        match self.fruit_type {
            FruitType::Apple => {
                ctx.set(self.location.x, self.location.y, WHITE, RED, to_cp437('.'))
            }
            FruitType::RottenApple => {
                ctx.set(self.location.x, self.location.y, WHITE, RED, to_cp437(','))
            }
        }
    }
}
