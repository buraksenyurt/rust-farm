use crate::prelude::*;

pub fn create_chest(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            asset_path: "/images/chest.png".to_string(),
        })
        .with(Chest {})
        .with(Movable {})
        .build();
}
