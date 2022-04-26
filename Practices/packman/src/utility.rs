use crate::prelude::*;

pub fn get_random_point(map: &Map, gen: &mut RandomNumberGenerator) -> (i32, i32, usize) {
    loop {
        let (x, y) = (gen.range(1, DISPLAY_WIDTH), gen.range(1, DISPLAY_HEIGHT));
        let index = map_to_index(x, y);
        if map.objects[index] != ObjectType::Floor {
            continue;
        }
        return (x, y, index);
    }
}

pub fn get_available_entry_point(map: &Map, gen: &mut RandomNumberGenerator) -> Point {
    loop {
        let (x, y, index) = get_random_point(&map, gen);
        if map.objects[index] != ObjectType::Floor {
            continue;
        }
        return Point::new(x, y);
    }
}
