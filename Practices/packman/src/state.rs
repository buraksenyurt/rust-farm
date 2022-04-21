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
            packy: Packy::new(map_builder.packy_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        for i in 0..4 {
            ctx.set_active_console(i);
            ctx.cls();
        }

        self.packy.move_to(ctx, &mut self.map);
        self.map.render(ctx);
        self.packy.render(ctx);
    }
}
