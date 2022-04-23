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

    pub fn move_to(&mut self, ctx: &mut BTerm, map: &mut Map) {
        if let Some(key) = ctx.key {
            let (x, y) = (self.location.x, self.location.y);

            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                VirtualKeyCode::Space => {
                    if map.bomb_count != 0 {
                        info!("CURRENT BOMB COUNT {}", map.bomb_count);
                        map.bomb_count -= 1;
                        let (u, d, l, r) = self.get_cells_around_packy();
                        self.blast_it(u, map);
                        self.blast_it(d, map);
                        self.blast_it(l, map);
                        self.blast_it(r, map);
                    }
                    Point::new(x, y)
                }
                _ => Point::zero(),
            };
            info!("PACKY CURRENT LOC -> {}:{}", x, y);
            let new_location = self.location + delta;
            info!(
                "MOVE ACTION FOR NEW LOCATION -> {}:{}",
                new_location.x, new_location.y
            );
            if let Some(index) = map.try_map_to_index(new_location) {
                match map.objects[index] {
                    ObjectType::Floor => {
                        info!("\t{}:{} is Floor", x, y);
                        info!("IT CAN GO TO -> {:?}", self.location);
                        self.location = new_location;
                    }
                    ObjectType::Apple(id) => {
                        info!("\t{}:{} is Apple", x, y);
                        info!("IT CAN GO TO -> {:?}", self.location);
                        self.location = new_location;
                        map.player_score += 10;
                        info!("+10 POINT. CURRENT SCORE IS {}", map.player_score);

                        map.apples[id].eated();
                        map.objects[index] = ObjectType::Floor;
                    }
                    ObjectType::RottenApple(id) => {
                        info!("\t{}:{} is Rotten Apple", x, y);
                        info!("IT CAN GO TO -> {:?}", self.location);
                        self.location = new_location;
                        map.player_score -= 5;
                        info!("-5 POINT. CURRENT SCORE IS {}", map.player_score);

                        map.roten_apples[id].eated();
                        map.objects[index] = ObjectType::Floor;
                    }
                    ObjectType::Wall(_) => info!("\t{}:{} is Wall", x, y),
                }
            }
        }
    }

    fn get_cells_around_packy(&self) -> (Point, Point, Point, Point) {
        let up_cell = Point::new(self.location.x, self.location.y - 1);
        let down_cell = Point::new(self.location.x, self.location.y + 1);
        let right_cell = Point::new(self.location.x + 1, self.location.y);
        let left_cell = Point::new(self.location.x - 1, self.location.y);

        info!(
            "Blasting Points ({:?}), ({:?}), ({:?}), ({:?})",
            up_cell, down_cell, right_cell, left_cell
        );
        (up_cell, down_cell, right_cell, left_cell)
    }

    fn blast_it(&mut self, p: Point, map: &mut Map) {
        if let Some(index) = map.try_map_to_index(p) {
            if let ObjectType::Wall(i) = map.objects[index] {
                info!("{} IS A WALL. BLAST IT", { index });
                map.walls[i].blasted();
                map.objects[index] = ObjectType::Floor
            }
        }
    }
}