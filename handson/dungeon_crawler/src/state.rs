use crate::prelude::*;

// State tutan veri yapımız
pub struct State {
    map: Map,
}

impl State {
    pub fn new() -> Self {
        Self { map: Map::new() }
    }
}

// Bracket-lib'den gelen Gamestate trait'ini uyguluyoruz.
// Böylece oyun motorunun tick fonksiyonelliğini state nesnesi için kurgulayabiliriz.
// Her tiklemede sahnenin çizilmesi, karakterlerin hareketi, kural kontrolleri vs yapabiliriz.
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // Sahneyi sil
        self.map.render(ctx); // Haritayı çiz
    }
}
