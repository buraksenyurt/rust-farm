use crate::state::State;
use bracket_lib::prelude::{main_loop, BError, BTermBuilder};

mod constants;
mod elder;
mod state;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_fps_cap(1.0)
        .with_title("Geometriks")
        .build()?;

    main_loop(context, State::new())
}
