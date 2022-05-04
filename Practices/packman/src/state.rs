use crate::prelude::*;

pub struct State {
    map: Map,
    packy: Packy,
    boss: Boss,
    mode: GameMode,
    tick_count: u16,
    tick_level: u16,
    end_state: EndState,
}

impl State {
    pub fn new() -> Self {
        warn!("New game setup");
        let mut gen = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut gen);
        Self {
            map: map_builder.map,
            packy: Packy::new(map_builder.packy_start),
            boss: Boss::new(map_builder.boss_start),
            mode: GameMode::Menu,
            tick_count: 0,
            tick_level: u16::from(BossLevel::Gentle),
            end_state: EndState::Nothing,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Menu;
        ctx.cls();
        ctx.print_centered(5, "PackyMan - Catch Apples");
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(11, "(O) Options");
        ctx.print_centered(14, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::O => self.options(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn options(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Options;
        ctx.cls();
        ctx.print_centered(3, "Boss Level");
        ctx.print_centered(5, "(F5) Smurf");
        ctx.print_centered(8, "(F6) Gentle");
        ctx.print_centered(11, "(F7) Monstrous");
        ctx.print_centered(13, "(ESC) Back");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::F5 => {
                    self.tick_level = u16::from(BossLevel::Smurf);
                    self.tick_count = 0;
                    self.main_menu(ctx);
                    info!("Tick Level {}", BossLevel::Smurf);
                }
                VirtualKeyCode::F6 => {
                    self.tick_level = u16::from(BossLevel::Gentle);
                    self.tick_count = 0;
                    self.main_menu(ctx);
                    info!("Tick Level {}", BossLevel::Gentle);
                }
                VirtualKeyCode::F7 => {
                    self.tick_level = u16::from(BossLevel::Monstrous);
                    self.tick_count = 0;
                    self.main_menu(ctx);
                    info!("Tick Level {}", BossLevel::Monstrous);
                }
                VirtualKeyCode::Escape => {
                    self.main_menu(ctx);
                }
                _ => {}
            }
        }
    }

    fn restart(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Playing;
        let mut gen = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut gen);
        self.map = map_builder.map;
        self.packy = Packy::new(map_builder.packy_start);
        self.boss = Boss::new(map_builder.boss_start);
        self.play(ctx);
    }

    fn play(&mut self, ctx: &mut BTerm) {
        if let Some(VirtualKeyCode::Escape) = ctx.key {
            self.main_menu(ctx)
        }

        if self.map.eated_apple_count == MAX_NUM_OF_APPLES {
            self.end_state = EndState::Winner;
            self.map.duration = Instant::now().duration_since(self.map.start_time);
            self.end_game(ctx);
            return;
        }

        for i in 0..5 {
            ctx.set_active_console(i);
            ctx.cls();
        }

        self.packy.move_to(ctx, &mut self.map);
        self.map.render(ctx);
        self.packy.render(ctx);

        self.tick_count += 1;
        if self.tick_count == self.tick_level {
            //info!("Tick Counts is {}", self.tick_count);
            self.tick_count = 0;
            self.boss.move_to(&mut self.map, &self.packy);
            if is_packy_catched(&self.packy, &self.boss) {
                warn!("Packy catched by boss");
                self.end_state = EndState::Loser;
                self.end_game(ctx);
                return;
            }
        }

        self.boss.render(ctx);
    }

    fn end_game(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::End;
        ctx.cls();
        ctx.print_centered(3, "The game is over.");
        match self.end_state {
            EndState::Winner => {
                ctx.print_centered(8, "WINNER");
                ctx.print_centered(
                    9,
                    format!(
                        "{} point in {} seconds",
                        self.map.player_score,
                        self.map.duration.as_secs()
                    ),
                );
            }
            EndState::Loser => ctx.print_centered(8, "LOSER"),
            _ => {}
        }
        ctx.print_centered(11, "(P) Play Again ;)");
        ctx.print_centered(13, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::Options => self.options(ctx),
            GameMode::End => self.end_game(ctx),
        }
    }
}
