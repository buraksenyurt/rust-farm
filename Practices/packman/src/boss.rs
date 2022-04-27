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

        let suggestions = vec![
            Point::new(x - 1, y),
            Point::new(x + 1, y),
            Point::new(x, y - 1),
            Point::new(x, y + 1),
        ];
        for suggestion in suggestions {
            if let Some(index) = map.try_map_to_index(suggestion) {
                match map.objects[index] {
                    ObjectType::Floor => {
                        info!("BOSS on the go -> {:?}", self.location);
                        self.location = suggestion;
                        break;
                    }
                    _ => {}
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
