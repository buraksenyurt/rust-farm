use crate::prelude::*;

// Oyuncuyu temsil eden veri yapımız
pub struct Adventurer {
    // Maceracımızın güncel koordinatlarını tutan alan
    pub location: Point,
}

impl Adventurer {
    pub fn new(location: Point) -> Self {
        Self { location }
    }

    // Oyunucuyu sahaya çizmek için kullanılan render fonksiyonu.
    // x,y bilgilerine göre siyah zemin üstüne beyaz renkte bir A harfi basmakta.
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.location.x,
            self.location.y,
            WHITE,
            BLACK,
            to_cp437('A'),
        )
    }
}
