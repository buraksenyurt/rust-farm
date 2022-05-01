use crate::game::Game;
use ggez::{conf, event, GameResult};
use std::path::PathBuf;

mod game;

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("game_1", "buraks")
        .window_setup(conf::WindowSetup::default().title("Sokoban Game"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(PathBuf::from("./resources"));

    let (ctx, event_loop) = context_builder.build()?;
    let game = Game {};
    event::run(ctx, event_loop, game);
}
