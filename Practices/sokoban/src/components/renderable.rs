use crate::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    path: String,
}
