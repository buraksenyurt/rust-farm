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
            YELLOW,
            WHITE,
            to_cp437('@'),
        )
    }

    pub fn move_to(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let (x, y) = (self.location.x, self.location.y);

            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };
            info!("PACKY CURRENT LOC -> {}:{}", x, y);
            let new_location = self.location + delta;
            info!(
                "MOVE ACTION FOR NEW LOCATION -> {}:{}",
                new_location.x, new_location.y
            );
            let index = map_to_index(new_location.x, new_location.y);
            match map.objects[index] {
                ObjectType::Floor => info!("\t{}:{} is Floor", x, y),
                ObjectType::Apple => info!("\t{}:{} is Apple", x, y),
                ObjectType::RottenApple => info!("\t{}:{} is Rotten Apple", x, y),
                ObjectType::Wall => info!("\t{}:{} is Wall", x, y),
            }
            if map.is_in_bounds(new_location) {
                self.location = new_location;
                info!("IT CAN GO TO -> {}:{}", self.location.x, self.location.y);
            }
        }
    }
}
