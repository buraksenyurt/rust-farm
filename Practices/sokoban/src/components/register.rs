use crate::prelude::*;

// Bileşenleri(components) oyun dünyası nesnesine kayıt etmek için kullanılan fonksiyon
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Chest>();
    world.register::<ChestSpot>();
    world.register::<Wall>();
}

// Basılan tuş olayları gibi genel kaynakları oyun dünyası nesnesine kayıt etmek için kullanılıyor
pub fn register_resources(world: &mut World) {
    world.insert(InputEvents::default())
}
