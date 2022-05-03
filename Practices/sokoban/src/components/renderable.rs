use crate::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub asset_path: String,
}
