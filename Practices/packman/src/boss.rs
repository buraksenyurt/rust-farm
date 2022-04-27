use crate::prelude::*;

pub struct Boss {
    pub location: Point,
}

impl Boss {
    pub fn new(location: Point) -> Self {
        Self { location }
    }

    pub fn move_to(&mut self, map: &mut Map) {
        let (x, y) = (self.location.x, self.location.y);
        //info!("Current boss location {:?}", self.location);
        let mut gen = RandomNumberGenerator::new();

        let suggestions = vec![
            Point::new(x - 1, y),
            Point::new(x + 1, y),
            Point::new(x, y - 1),
            Point::new(x, y + 1),
        ];
        loop {
            let index = gen.range(0, 4);
            if let Some(i) = try_map_to_index(suggestions[index]) {
                match map.objects[i] {
                    ObjectType::Floor | ObjectType::Apple(_) => {
                        self.location = suggestions[index];
                        return;
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
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
