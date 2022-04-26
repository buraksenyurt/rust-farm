use crate::prelude::*;

pub struct State {
    map: Map,
    packy: Packy,
    boss: Boss,
    mode: GameMode,
}

impl State {
    pub fn new() -> Self {
        let mut gen = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut gen);
        Self {
            map: map_builder.map,
            packy: Packy::new(map_builder.packy_start),
            boss: Boss::new(map_builder.boss_start),
            mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Menu;
        ctx.cls();
        ctx.print_centered(5, "PackyMan - Catch Apples");
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(11, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self, ctx: &mut BTerm) {
        let mut gen = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut gen);
        self.map = map_builder.map;
        self.packy = Packy::new(map_builder.packy_start);
        self.mode = GameMode::Playing;
        self.play(ctx);
    }

    fn play(&mut self, ctx: &mut BTerm) {
        if let Some(VirtualKeyCode::Escape) = ctx.key {
            self.main_menu(ctx)
        }

        for i in 0..5 {
            ctx.set_active_console(i);
            ctx.cls();
        }

        self.packy.move_to(ctx, &mut self.map);
        self.map.render(ctx);
        self.packy.render(ctx);
        self.boss.render(ctx);
    }

    // fn end_game(&mut self, ctx: &mut BTerm) {
    //     ctx.cls();
    //     ctx.print_centered(3, "The game is over!");
    //     ctx.print_centered(8, "(P) Play Again");
    //     ctx.print_centered(11, "(Q) Quit");
    //
    //     if let Some(key) = ctx.key {
    //         match key {
    //             VirtualKeyCode::P => self.restart(ctx),
    //             VirtualKeyCode::Q => ctx.quitting = true,
    //             _ => {}
    //         }
    //     }
    //     self.mode = GameMode::End;
    // }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            //GameMode::End => self.end_game(ctx),
        }
    }
}
