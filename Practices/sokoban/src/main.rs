use prelude::*;

mod components;
mod game;

pub mod prelude {
    pub use crate::components::*;
    pub use crate::game::Game;
    pub use ggez::conf::*;
    pub use ggez::*;
    pub use specs::*;
    pub use std::path::PathBuf;
}

fn main() -> GameResult {
    let context_builder = ContextBuilder::new("game_1", "buraks")
        .window_setup(WindowSetup::default().title("Sokoban Game"))
        .window_mode(WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(PathBuf::from("./resources"));

    let (ctx, event_loop) = context_builder.build()?;
    let game = Game {};
    event::run(ctx, event_loop, game);
}
