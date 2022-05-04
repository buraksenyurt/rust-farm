use crate::prelude::*;

pub struct Boss {
    pub location: Point,
}

impl Boss {
    pub fn new(location: Point) -> Self {
        Self { location }
    }

    fn get_suggestions(&self, packy: &Packy, boss: &Boss) -> Vec<Point> {
        let mut suggestions: Vec<Point> = vec![];
        let (x, y) = (boss.location.x, boss.location.y);
        if packy.location.x < x {
            suggestions.push(Point::new(x - 1, y));
        } else if packy.location.x > x {
            suggestions.push(Point::new(x + 1, y));
        } else if packy.location.y < y {
            suggestions.push(Point::new(x, y - 1));
        } else if packy.location.y > y {
            suggestions.push(Point::new(x, y + 1));
        }

        suggestions
    }

    pub fn move_to(&mut self, map: &mut Map, packy: &Packy) {
        //let (x, y) = (self.location.x, self.location.y);
        //info!("Current boss location {:?}", self.location);
        //let mut gen = RandomNumberGenerator::new();
        // let suggestions = vec![
        //     Point::new(x - 1, y),
        //     Point::new(x + 1, y),
        //     Point::new(x, y - 1),
        //     Point::new(x, y + 1),
        // ];
        let suggestions = self.get_suggestions(packy, self);

        //loop {
        //let index = gen.range(0, 4);
        for suggestion in suggestions {
            if let Some(i) = try_map_to_index(suggestion) {
                match map.objects[i] {
                    ObjectType::Floor | ObjectType::Wall(_) => {
                        self.location = suggestion;
                        return;
                    }
                    ObjectType::Apple(id) => {
                        self.location = suggestion;
                        if !map.apples[id].is_eated() {
                            map.apples[id].eated();
                            map.eated_apple_count += 1;
                            map.player_score -= RED_APPLE_POINT_FOR_BOSS;
                            map.objects[i] = ObjectType::Floor;
                        }
                        return;
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
        //}
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
