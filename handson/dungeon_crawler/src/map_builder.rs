use crate::prelude::*;

// Zindandaki maksimum oda sayısını tuttuğumuz sabit değişken.
const MAX_NUM_OF_ROOMS: usize = 24;

// MapBuilder veri modeli
// Kendi haritasını tutar. Odalar için Bracket-Lib'den gelen ve dikdörtgen nesneleri
// işaret eden Rect tipinden bir vector serisi kullanır.
// Oyuncunun haritaya girdiği konum bilgisi de Point türünden saklanır.
pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub hero_start: Point,
}

impl MapBuilder {
    // Bu fonksiyon ile MapBuilder'ın tuttuğu harita üzerindeki alanlar parametre olarak gelen
    // TileType ile kaplanır. Fonksiyon odaları ve duvarları hazırlamakta kolaylık sağlayacaktır.
    // MapBuilder'ın sahip olduğu map'in tiles içeriğinde değişiklik yapılması gerektiğinden
    // iter_mut ile iterasyon mutable şekilde ele alınmıştır. * ile de-reference işlemi uygulanır.
    // Yani &tile referansının işaret ettiği değer çekilir.
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    // Bu fonksiyon maksimum sayıda odayı haritadaki rastgele konumlarına yerleştirmektedir.
    fn build_rooms(&mut self, gen: &mut RandomNumberGenerator) {
        // Odalar için ayrılan sayı dolana kadar devam edecek bir while döngüsü.
        while self.rooms.len() < MAX_NUM_OF_ROOMS {
            // Oyun sahası içerisine denk gelecek rastgele x,y koordinatları alınır.
            let (x, y) = (
                gen.range(1, SCHENE_WIDTH - 10),
                gen.range(1, SCHENE_HEIGHT - 10),
            );
            // 2,10 arasında değişen değerlerde odanın genişlik ve yükselik değerleri alınır.
            let (w, h) = (gen.range(2, 10), gen.range(2, 10));
            // Bir dikdörtgen örneklenir
            let room = Rect::with_size(x, y, w, h);

            // maksimum sayıda oda üretirken, üretilen bir odanın üstüne başka birisinin
            // gelmemesi gerekiyor. Bunun kontrolü için overlap değişkeni kullanılabilir.
            let mut overlap = false;
            // hali hazırda var olan tüm odalara yeni oluşturulan oda ile intersect fonksiyonu
            // üstünden kontrol ediyoruz.
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            // Eğer yeni oda diğerleri ile çakışmıyorsa haritadaki konumlarını tiles üstündeki
            // ilgili indekse konumlandırıyoruz.
            if !overlap {
                // Odanın tüm noktalarını gezen bir döngü açılıyor.
                room.for_each(|p| {
                    // Odanın o noktası oyun sahası içinde kalıyorsa,
                    if p.x > 0 && p.x < SCHENE_WIDTH && p.y > 0 && p.y < SCHENE_HEIGHT {
                        // x,y karşılığı olan map indisi bulunuyor
                        let index = map_to_index(p.x, p.y);
                        // ve bu x,y koordinatı Duvar tipi ile işaretleniyor
                        self.map.tiles[index] = TileType::Floor;
                    }
                });
                // Odanın noktaları haritada işaretlendikten sonra rooms vektörüne ekleniyor.
                self.rooms.push(room)
            }
        }
    }
}
