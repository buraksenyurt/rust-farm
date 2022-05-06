use crate::prelude::*;

// Bileşenleri(components) oyun dünyası nesnesine kayıt etmek için kullanılan fonksiyon
pub fn register_components(world: &mut World) {
    world.register::<Position>(); // Bir konumu olan varlıkların bileşeni
    world.register::<Renderable>(); // Ekrana çizilebilir varlıkların bileşeni
    world.register::<Player>(); // Oyuncu bileşeni
    world.register::<Chest>(); // Sandık bileşeni
    world.register::<ChestSpot>(); // Kilitli sandık bileşeni
    world.register::<Wall>(); // Duvar bileşeni
    world.register::<Movable>(); // Hareket edebilir nesneler için tanımlanmış bileşen
    world.register::<Immovable>(); // Hareket ettirilemez nesneler için tanımlanmış bileşen
}

// Basılan tuş olayları gibi genel kaynakları oyun dünyası nesnesine kayıt etmek için kullanılıyor
pub fn register_resources(world: &mut World) {
    world.insert(InputEvents::default())
}
