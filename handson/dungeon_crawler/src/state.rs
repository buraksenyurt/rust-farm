use crate::prelude::*;

// State tutan veri yapısı.
// Haritayı, oyuncuyu ve kamerayı tutmakta.
pub struct State {
    map: Map,
    hero: Adventurer,
    visor: Camera,
}

impl State {
    pub fn new() -> Self {
        let mut gen = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut gen);
        Self {
            map: map_builder.map,
            hero: Adventurer::new(map_builder.hero_start),
            visor: Camera::new(map_builder.hero_start),
        }
    }
}

// Bracket-lib'den gelen Gamestate trait'ini uyguluyoruz.
// Böylece oyun motorunun tick fonksiyonelliğini state nesnesi için kurgulayabiliriz.
// Her tiklemede sahnenin çizilmesi, karakterlerin hareketi, kural kontrolleri vs yapabiliriz.
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0); // sahneye ilk console layer'ı ekle
        ctx.cls(); // Sahneyi sil
        ctx.set_active_console(1); // İkinci console layer'ı da ekle
        ctx.cls();
        self.hero.go(ctx, &self.map, &mut self.visor); // 1,1 konumuna oyuncuyu ekle
        self.map.render(ctx, &self.visor); // Haritayı kamerayı kullanarak çiz
        self.hero.render(ctx, &self.visor); // Oyuncuyu çiz
    }
}
