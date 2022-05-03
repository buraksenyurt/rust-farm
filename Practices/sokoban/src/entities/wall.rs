use crate::prelude::*;

pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            asset_path: "/images/wall.png".to_string(),
        })
        .with(Wall {})
        .build();
}
