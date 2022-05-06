use crate::prelude::*;

pub fn create_chest_spot(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable {
            asset_path: "/images/chest_spot.png".to_string(),
        })
        .with(ChestSpot {})
        .with(Immovable {})
        .build();
}
