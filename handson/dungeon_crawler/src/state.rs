use crate::prelude::*;

// State tutan veri yapımız
pub struct State {
    map: Map,
    hero: Adventurer,
}

impl State {
    pub fn new() -> Self {
        let mut gen = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut gen);
        Self {
            map: map_builder.map,
            hero: Adventurer::new(map_builder.hero_start),
        }
    }
}

// Bracket-lib'den gelen Gamestate trait'ini uyguluyoruz.
// Böylece oyun motorunun tick fonksiyonelliğini state nesnesi için kurgulayabiliriz.
// Her tiklemede sahnenin çizilmesi, karakterlerin hareketi, kural kontrolleri vs yapabiliriz.
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // Sahneyi sil
        self.hero.go(ctx, &self.map); // 1,1 konumuna oyuncuyu ekle
        self.map.render(ctx); // Haritayı çiz
        self.hero.render(ctx); // Oyuncuyu çiz
    }
}
