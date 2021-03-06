use crate::prelude::*;

pub fn get_random_point(map: &Map, gen: &mut RandomNumberGenerator) -> (i32, i32, usize) {
    loop {
        let (x, y) = (gen.range(0, DISPLAY_WIDTH), gen.range(1, DISPLAY_HEIGHT));
        let index = map_to_index(x, y);
        if map.objects[index] != ObjectType::Floor {
            continue;
        }
        return (x, y, index);
    }
}

pub fn get_available_entry_point(map: &Map, gen: &mut RandomNumberGenerator) -> Point {
    loop {
        let (x, y, index) = get_random_point(map, gen);
        if map.objects[index] != ObjectType::Floor || !is_cell_suitable(map, x, y) {
            continue;
        }
        return Point::new(x, y);
    }
}

fn is_cell_suitable(map: &Map, x: i32, y: i32) -> bool {
    if map.objects[map_to_index(x - 1, y)] == ObjectType::Floor
        || map.objects[map_to_index(x + 1, y)] == ObjectType::Floor
        || map.objects[map_to_index(x, y - 1)] == ObjectType::Floor
        || map.objects[map_to_index(x, y + 1)] == ObjectType::Floor
    {
        return true;
    }
    false
}

pub fn find_warp_point(map: &Map, p: Point) -> Point {
    let mut gen = RandomNumberGenerator::new();
    loop {
        let jump_point = Point::new(gen.range(-10, 10), gen.range(-10, 10));
        let candidate = p + jump_point;
        if !is_in_bounds(candidate) {
            continue;
        }
        let index = map_to_index(candidate.x, candidate.y);
        if map.objects[index] != ObjectType::Floor {
            continue;
        }
        return jump_point;
    }
}

pub fn map_to_index(x: i32, y: i32) -> usize {
    ((y * DISPLAY_WIDTH) + x) as usize
}

pub fn is_in_bounds(point: Point) -> bool {
    (point.x >= 0 && point.x < DISPLAY_WIDTH) && (point.y >= 1 && point.y < DISPLAY_HEIGHT)
}

pub fn try_map_to_index(point: Point) -> Option<usize> {
    if !is_in_bounds(point) {
        None
    } else {
        Some(map_to_index(point.x, point.y))
    }
}

pub fn is_packy_catched(packy: &Packy, boss: &Boss) -> bool {
    packy.location == boss.location
}
