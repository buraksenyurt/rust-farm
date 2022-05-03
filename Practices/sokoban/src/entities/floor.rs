use crate::prelude::*;

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable {
            asset_path: "/images/floor.png".to_string(),
        })
        .build();
}
