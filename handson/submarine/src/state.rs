use super::player::Player;
use crate::constant::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::game_mode::GameMode;
use crate::level::Level;
use crate::rock::Rock;
use bracket_lib::prelude::{BTerm, GameState, VirtualKeyCode, BLACK, TURQUOISE};

/// Oyunun anlık durumuna ait görüntüsünü(snapshot) tutan nesnedir.
pub struct State {
    // Oyunun anlık durumunda oyunun hangi bölümünde olduğumuzu,
    // oyunun çerçeve hızını ve oyuncu bilgilerini saklarız.
    pub mode: GameMode,
    pub frame_time: f32,
    pub player: Player,
    pub rock: Rock,
    pub score: i32,
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Menu,
            frame_time: 0.0,
            player: Player::new(0, 25),
            rock: Rock::new(),
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Menu;
        ctx.cls(); // Ekranı temizler
                   // Menu seçeneklerini aşağıdaki gibi merkezi olarak yerleştirebiliriz.
        ctx.print_centered(5, "Submarine - Defend Your Sea");
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(11, "(C) Configuration");
        ctx.print_centered(15, "(Q) Quit");

        // Kullanıcının seçimini tuşa basma usulü ile yakalayabiliriz.
        // P tuşuna basıldıysa restart fonksiyonunu çağırarak oyunun modunu Playing'e aldırmaktayız.
        // C tuşuna basıldıysa oyun seviyesini belirleyen konfigurasyon kısmına geçilir.
        // Q tuşuna basıldığında da tahmin edileceği üzere oyun sonlanır.
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::C => self.config(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(TURQUOISE);
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => self.up(),
                VirtualKeyCode::Down => self.down(),
                VirtualKeyCode::Left => self.left(),
                VirtualKeyCode::Right => self.right(),
                VirtualKeyCode::R => self.restart(),
                VirtualKeyCode::Escape => self.main_menu(ctx),
                _ => {}
            }
        }
        self.player.render(ctx);
        self.rock = self.rock.forward(self.rock.x - 1, self.rock.y);
        self.rock.render(ctx);
        if self.rock.x < 0 {
            self.rock = Rock::new();
            self.rock.render(ctx);
        }
        ctx.print_color(1, 1, TURQUOISE, BLACK, &format!("Hit {}", self.score));

        self.collision_check();
    }

    /*
       y
       |
       |
       |
       |
     (0.0)_ _ _ _ _ _ _ > x
    */

    pub fn collision_check(&mut self) {
        if self.player.x == self.rock.x
            && (self.player.y > self.rock.y - 5 && self.player.y < self.rock.y + 5)
        {
            self.score += 5;
        }
    }

    fn config(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Config;
        ctx.cls();
        ctx.print_centered(3, "Choose Level");
        ctx.print_centered(5, "(F5) Easy");
        ctx.print_centered(8, "(F6) Medium");
        ctx.print_centered(11, "(F7) Hard");
        ctx.print_centered(13, "(ESC) Main Menu");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::F5 => self.player.level = Level::Easy,
                VirtualKeyCode::F6 => self.player.level = Level::Medium,
                VirtualKeyCode::F7 => self.player.level = Level::Hard,
                VirtualKeyCode::Escape => self.main_menu(ctx),
                _ => {}
            }
            self.score = 0;
        }
    }
    fn end_game(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(3, "The game is over!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(11, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
        self.player.render(ctx);
        ctx.print_color(1, 1, TURQUOISE, BLACK, &format!("Hit {}", self.score));
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.rock = Rock::new();
    }

    pub fn up(&mut self) {
        self.player.y -= i32::from(&self.player.level);
        if self.player.y <= 2 {
            self.player.y = 2;
        }
    }

    pub fn down(&mut self) {
        self.player.y += i32::from(&self.player.level);
        if self.player.y >= SCREEN_HEIGHT {
            self.player.y = SCREEN_HEIGHT - 1;
        }
    }

    pub fn right(&mut self) {
        self.player.x += i32::from(&self.player.level);
        if self.player.x >= SCREEN_WIDTH {
            self.player.x = SCREEN_WIDTH - 5
        }
    }

    pub fn left(&mut self) {
        self.player.x -= i32::from(&self.player.level);
        if self.player.x <= 2 {
            self.player.x = 2;
        }
    }
}

// Oyunda anlık durumu değiştirecek türden bir tick fonksiyonu olmalıdır.
// Oyun durumunu tutacağımız State veri yapısı için bu özelliği GameState trait'i ile State nesnemiz için uygulamaktayız.
impl GameState for State {
    // Oyun motoru ile oyun arasındaki köprüyü kuran fonksiyondur.
    fn tick(&mut self, ctx: &mut BTerm) {
        // tick fonksiyonu oyunun o anki moduna göre bir aksiyon alır.
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::Config => self.config(ctx),
            GameMode::End => self.end_game(ctx),
        }
    }
}
