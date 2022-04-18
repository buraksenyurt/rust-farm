use crate::prelude::*;

pub struct Packy {
    pub location: Point,
}

impl Packy {
    pub fn new(location: Point) -> Self {
        Self { location }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.set(
            self.location.x,
            self.location.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        )
    }

    pub fn move_to(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };
            let new_location = self.location + delta;
            if map.want_enter_tile(new_location) {
                self.location = new_location;
            }
        }
    }
}
