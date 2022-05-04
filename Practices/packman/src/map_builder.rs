use crate::prelude::*;

pub struct MapBuilder {
    pub map: Map,
    pub packy_start: Point,
    pub boss_start: Point,
    pub ghost_start: Point,
}

impl MapBuilder {
    pub fn new(gen: &mut RandomNumberGenerator) -> Self {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            packy_start: Point::zero(),
            boss_start: Point::zero(),
            ghost_start: Point::zero(),
        };
        map_builder.fill_ground(ObjectType::Floor);

        map_builder.add_walls(gen);
        map_builder.add_apples(gen);
        map_builder.add_rotten_apples(gen);
        map_builder.packy_start = get_available_entry_point(&map_builder.map, gen);
        map_builder.boss_start = get_available_entry_point(&map_builder.map, gen);
        map_builder.ghost_start = get_available_entry_point(&map_builder.map, gen);
        map_builder
    }

    fn fill_ground(&mut self, object: ObjectType) {
        self.map.objects.iter_mut().for_each(|t| *t = object);
    }

    fn add_walls(&mut self, gen: &mut RandomNumberGenerator) {
        info!("CREATING {} WALL", MAX_NUM_OF_WALLS);

        for i in 0..MAX_NUM_OF_WALLS {
            let (x, y, index) = get_random_point(&self.map, gen);
            info!("{}:{} -> WALL", x, y);
            let wall = Wall::new(Point { x, y });
            self.map.walls.push(wall);
            info!("[{}] CHANGED TO WALL", index);
            self.map.objects[index] = ObjectType::Wall(i);
        }
    }

    fn add_apples(&mut self, gen: &mut RandomNumberGenerator) {
        for i in 0..MAX_NUM_OF_APPLES {
            let (x, y, index) = get_random_point(&self.map, gen);
            info!("{}:{} -> APPLE", x, y);
            let apple = Apple::new(Point { x, y }, FruitType::Apple);
            self.map.apples.push(apple);
            info!("[{}] CHANGED TO APPLE", index);
            self.map.objects[index] = ObjectType::Apple(i);
        }
    }

    fn add_rotten_apples(&mut self, gen: &mut RandomNumberGenerator) {
        for i in 0..MAX_NUM_OF_ROTTEN_APPLES {
            let (x, y, index) = get_random_point(&self.map, gen);
            info!("{}:{} -> ROTTEN APPLE", x, y);
            let rotten_apple = Apple::new(Point { x, y }, FruitType::RottenApple);
            self.map.roten_apples.push(rotten_apple);
            info!("[{}] CHANGED TO ROTTEN_APPLE", index);
            self.map.objects[index] = ObjectType::RottenApple(i);
        }
    }
}
