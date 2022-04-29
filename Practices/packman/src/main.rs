use crate::prelude::*;

mod apple;
mod boss;
mod boss_level;
mod constants;
mod floor;
mod game_mode;
mod map;
mod map_builder;
mod object_type;
mod packy;
mod state;
mod utility;
mod wall;

mod prelude {
    pub use crate::apple::*;
    pub use crate::boss::*;
    pub use crate::boss_level::*;
    pub use crate::constants::*;
    pub use crate::floor::*;
    pub use crate::game_mode::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::object_type::*;
    pub use crate::packy::*;
    pub use crate::state::*;
    pub use crate::utility::*;
    pub use crate::wall::*;
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
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_SOURCE)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_SOURCE)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_SOURCE)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_SOURCE)
        .build()?;

    main_loop(context, State::new())
}
