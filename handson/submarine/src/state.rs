use crate::game_mode::GameMode;
use bracket_lib::prelude::{BTerm, GameState, VirtualKeyCode};

/// Oyunun anlık durumuna ait görüntüsünü(snapshot) tutan nesnedir.
pub struct State {
    pub mode: GameMode,
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Menu,
        }
    }

    // Oyunun menüsünü oluşturan fonksiyon
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // Ekranı temizler
                   // Menu seçeneklerini aşağıdaki gibi merkezi olarak yerleştirebiliriz.
        ctx.print_centered(5, "Submarine - Defend Your Sea");
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(11, "(Q) Quit");

        // Kullanıcının seçimini tuşa basma usulü ile yakalayabiliriz.
        // P tuşuna basıldıysa restart fonksiyonunu çağırarak oyunun modunu Playing'e aldırmaktayız.
        // Q tuşuna basıldığında da tahmin edileceği üzere oyun sonlanır.
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        todo!()
    }

    fn end_game(&mut self, ctx: &mut BTerm) {
        todo!()
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing
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
            GameMode::End => self.end_game(ctx),
        }

        // // Oyun penceresi ile iletişim için context'e ihtiyacımız vardır.
        // // BTerm türünden olan ctx ile ekranı temizleyebilir
        // // üzerine yeni nesneler konumlandırabilir ve başka etkileşimleri sağlayabiliriz.
        // ctx.cls(); // Ekranı temizle
        //            // Koordinat sistemine göre sol üst köşeye bir mesaj yazdırıyoruz.
        //            // Sol üst köşe 0,0 konumu iken bracket-lib'e göre sağ alt köşe 79,49 konumudur.
        // ctx.print(1, 1, "1,1 konumundan merhaba");
    }
}
