use prelude::*;

mod components;
mod constants;
mod entities;
mod game;
mod map;
mod mocker;
mod resources;
mod systems;

pub mod prelude {
    pub use crate::components::*;
    pub use crate::constants::*;
    pub use crate::entities::*;
    pub use crate::game::Game;
    pub use crate::input_events::*;
    pub use crate::map::*;
    pub use crate::mocker::*;
    pub use crate::resources::*;
    pub use crate::systems::*;
    pub use ggez::conf::*;
    pub use ggez::event::*;
    pub use ggez::*;
    pub use glam::*;
    pub use graphics::*;
    pub use log::*;
    pub use specs::world::Index;
    pub use specs::*;
    pub use std::collections::HashMap;
    pub use std::path::PathBuf;
}

fn main() -> GameResult {
    let _ = env_logger::init();

    // Yeni bir oyun dünyası nesnesi oluşturduk
    let mut world = World::new();
    // bileşenlerin kayıt işlemlerini üstlenen fonksiyonu çağırdık
    register_components(&mut world);
    // kaynakların kayıt işlemlerini üstlenen fonksiyonu çağırdık
    register_resources(&mut world);

    // test amaçlı birkaç entity'i ekran çizderebiliriz.
    //create_test_entites(&mut world);
    init_first_level(&mut world);

    // oyuna ait context nesnesi ve ana motor döngüsü oluşturuldu
    let context_builder = ContextBuilder::new("game_1", "buraks")
        .window_setup(WindowSetup::default().title("Sokoban Game"))
        .window_mode(WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(PathBuf::from("./resources"));

    let (ctx, event_loop) = context_builder.build()?;
    // Oyun nesnemiz world'ü barındıran bir state nesnesi olarak düşünülebilir.
    let game = Game { world };
    // oyunu başlattığımız yer. Context'i ana oyun döngüsünü ve state nesnesini parametre olarak almakta.
    run(ctx, event_loop, game);
}
