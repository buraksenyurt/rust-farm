use crate::prelude::*;

pub fn create_test_entites(world: &mut World) {
    create_chest(world, Position { x: 0, y: 0, z: 0 });
    create_floor(world, Position { x: 1, y: 0, z: 0 });
    create_player(world, Position { x: 3, y: 5, z: 0 });
    create_wall(world, Position { x: 1, y: 3, z: 0 });
    create_wall(world, Position { x: 2, y: 3, z: 0 });
    create_wall(world, Position { x: 3, y: 3, z: 0 });
    create_chest(world, Position { x: 4, y: 4, z: 0 });
    create_chest_spot(world, Position { x: 5, y: 4, z: 0 });
}
