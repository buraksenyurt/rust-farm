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
    // x,y bilgilerine göre @ sembolünün karşılığı olan figürü, builder'da belirttiğimiz
    // png kaynağından bulup basmakta. Konumlandırma kameranın bakış açısına göre yapılıyor.
    pub fn render(&self, ctx: &mut BTerm, visor: &Camera) {
        ctx.set_active_console(1);
        ctx.set(
            self.location.x - visor.left_x,
            self.location.y - visor.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        )
    }

    // go fonksiyonunda basılan yön tuşlarına göre oyuncunun,
    // oyun sahasındaki yeni bir konuma 1er birim hareketinin kontrolü söz konusu.
    // Öncelikle Context üstünden basılan tuş yakalanıyor.
    // Tuşun yönüne göre x,y değerleri için birer fark belirleniyor.
    // Yeni konum bilgisi hareket etmek için müsait mi can_enter_tile fonksiyonu ile bakılıyor.
    // Müsaitlik varsa oyuncunun yeni pozisyonu delta birimi kadar artırılan yeni konum oluyor.
    // Bu işlemin arkasında kamera açısının da değiştirilmesi için on_move fonksiyonu çağırılıyor.
    pub fn go(&mut self, ctx: &mut BTerm, map: &Map, visor: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };
            let new_location = self.location + delta;
            if map.can_enter_tile(new_location) {
                self.location = new_location;
                visor.on_move(new_location);
            }
        }
    }
}
