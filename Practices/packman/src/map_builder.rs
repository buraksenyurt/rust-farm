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
        map_builder.fill(ObjectType::Floor);

        map_builder.add_walls(gen);
        map_builder.add_apples(gen);
        map_builder.add_rotten_apples(gen);

        map_builder
    }
    fn fill(&mut self, object: ObjectType) {
        self.map.objects.iter_mut().for_each(|t| *t = object);
    }

    fn add_walls(&mut self, gen: &mut RandomNumberGenerator) {
        info!("CREATING {} WALL", MAX_NUM_OF_WALLS);

        for _ in 0..MAX_NUM_OF_WALLS {
            let (x, y) = (
                gen.range(1, DISPLAY_WIDTH),
                gen.range(1, DISPLAY_HEIGHT),
            );
            info!("{}:{} -> WALL", x, y);
            let wall = Wall::new(Point { x, y });
            self.map.walls.push(wall);
        }
    }

    fn add_apples(&mut self, gen: &mut RandomNumberGenerator) {
        for _ in 0..MAX_NUM_OF_APPLES {
            let (x, y) = (
                gen.range(1, DISPLAY_WIDTH),
                gen.range(1, DISPLAY_HEIGHT),
            );
            info!("{}:{} -> APPLE", x, y);
            let apple = Apple::new(Point { x, y });
            self.map.apples.push(apple);
        }
    }

    fn add_rotten_apples(&mut self, gen: &mut RandomNumberGenerator) {
        for _ in 0..MAX_NUM_OF_ROTTEN_APPLES {
            let (x, y) = (
                gen.range(1, DISPLAY_WIDTH),
                gen.range(1, DISPLAY_HEIGHT),
            );
            info!("{}:{} -> ROTTEN APPLE", x, y);
            let rotten_apple = RottenApple::new(Point { x, y });
            self.map.roten_apples.push(rotten_apple);
        }
    }
}
