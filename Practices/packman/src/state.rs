use crate::prelude::*;

pub struct State {
    map: Map,
    packy: Packy,
}

impl State {
    pub fn new() -> Self {
        let mut gen = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut gen);
        Self {
            map: map_builder.map,
            packy: Packy::new(map_builder.pack_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.packy.move_to(ctx, &self.map);
        self.map.render(ctx);
        self.packy.render(ctx);
    }
}
