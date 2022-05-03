use crate::state::State;
use bracket_lib::prelude::{main_loop, BError, BTermBuilder};

mod state;
mod constants;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Geometriks")
        .build()?;

    main_loop(context, State{})
}
