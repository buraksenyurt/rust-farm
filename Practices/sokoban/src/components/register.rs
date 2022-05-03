use crate::prelude::*;

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Chest>();
    world.register::<ChestSpot>();
    world.register::<Wall>();
}
