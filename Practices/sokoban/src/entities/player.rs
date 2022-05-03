use crate::prelude::*;

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            asset_path: "/images/player.png".to_string(),
        })
        .with(Player {})
        .build();
}
