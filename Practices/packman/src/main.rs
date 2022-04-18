use crate::prelude::*;

mod apple;
mod map;
mod map_builder;
mod packy;
mod rotten_apple;
mod state;

mod prelude {
    pub const DISPLAY_WIDTH: i32 = 40;
    pub const DISPLAY_HEIGHT: i32 = 25;
    pub const MAX_NUM_OF_APPLES: usize = 10;
    pub const MAX_NUM_OF_ROTTEN_APPLES: usize = 5;
    pub use crate::apple::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::packy::*;
    pub use crate::rotten_apple::*;
    pub use crate::state::*;
    pub use bracket_lib::prelude::*;
    pub const FONT_SOURCE: &str = "mapfonts.png";
    pub use log::*;
}
fn main() -> BError {
    let _ = env_logger::init();

    let context = BTermBuilder::new()
        .with_title("Packy Man")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font(FONT_SOURCE, 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_SOURCE)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_SOURCE)
        .build()?;

    main_loop(context, State::new())
}
