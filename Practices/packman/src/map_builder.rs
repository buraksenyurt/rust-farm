use crate::prelude::*;

pub struct MapBuilder {
    pub map: Map,
    pub packy_start: Point,
}

impl MapBuilder {
    pub fn new(gen: &mut RandomNumberGenerator) -> Self {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            packy_start: Point::zero(),
        };
        map_builder.fill_ground(ObjectType::Floor);

        map_builder.add_walls(gen);
        map_builder.add_apples(gen);
        map_builder.add_rotten_apples(gen);
        map_builder.packy_start = Self::get_available_entry_point(&mut map_builder, gen);
        map_builder
    }

    fn fill_ground(&mut self, object: ObjectType) {
        self.map.objects.iter_mut().for_each(|t| *t = object);
    }

    fn add_walls(&mut self, gen: &mut RandomNumberGenerator) {
        info!("CREATING {} WALL", MAX_NUM_OF_WALLS);

        for _ in 0..MAX_NUM_OF_WALLS {
            let (x, y, index) = self.get_random_point(gen);
            info!("{}:{} -> WALL", x, y);
            let wall = Wall::new(Point { x, y });
            self.map.walls.push(wall);
            info!("[{}] CHANGED TO WALL", index);
            self.map.objects[index] = ObjectType::Wall;
        }
    }

    fn add_apples(&mut self, gen: &mut RandomNumberGenerator) {
        for _ in 0..MAX_NUM_OF_APPLES {
            let (x, y, index) = self.get_random_point(gen);
            info!("{}:{} -> APPLE", x, y);
            let apple = Apple::new(Point { x, y });
            self.map.apples.push(apple);
            info!("[{}] CHANGED TO APPLE", index);
            self.map.objects[index] = ObjectType::Apple;
        }
    }

    fn add_rotten_apples(&mut self, gen: &mut RandomNumberGenerator) {
        for _ in 0..MAX_NUM_OF_ROTTEN_APPLES {
            let (x, y, index) = self.get_random_point(gen);
            info!("{}:{} -> ROTTEN APPLE", x, y);
            let rotten_apple = RottenApple::new(Point { x, y });
            self.map.roten_apples.push(rotten_apple);
            info!("[{}] CHANGED TO ROTTEN_APPLE", index);
            self.map.objects[index] = ObjectType::RottenApple;
        }
    }

    fn get_random_point(&mut self, gen: &mut RandomNumberGenerator) -> (i32, i32, usize) {
        loop {
            let (x, y) = (gen.range(1, DISPLAY_WIDTH), gen.range(1, DISPLAY_HEIGHT));
            let index = map_to_index(x, y);
            if self.map.objects[index] != ObjectType::Floor {
                continue;
            }
            return (x, y, index);
        }
    }

    fn get_available_entry_point(&mut self, gen: &mut RandomNumberGenerator) -> Point {
        loop {
            let (x, y, index) = self.get_random_point(gen);
            if self.map.objects[index] != ObjectType::Floor {
                continue;
            }
            return Point::new(x, y);
        }
    }
}
