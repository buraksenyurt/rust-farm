use crate::prelude::*;

mod map;
mod map_builder;
mod packy;
mod state;

mod prelude {
    pub const SCHENE_WIDTH: i32 = 80;
    pub const SCHENE_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCHENE_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCHENE_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::packy::*;
    pub use crate::state::*;
    pub use bracket_lib::prelude::*;
    pub const FONT_SOURCE: &str = "mapfonts.png";
}
fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
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
