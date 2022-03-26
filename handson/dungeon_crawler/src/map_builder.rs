use crate::prelude::*;
use std::cmp::max;

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
    // Yapıcı metot, MapBuilder içinde tanımlı private fonksiyonları kullanarak oyun sahasına
    // odaları ve koridorları çiziyor.
    pub fn new(gen: &mut RandomNumberGenerator) -> Self {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            hero_start: Point::zero(),
        };
        map_builder.fill(TileType::Wall);
        map_builder.build_rooms(gen);
        map_builder.build_tunnels(gen);
        map_builder.hero_start = map_builder.rooms[0].center();
        map_builder
    }

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

    // Dikey koridor çizmek için kullanılan yardımcı fonksiyon.
    fn build_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        // Kullanılacak modüle bildirimlerini buradaki gibi fonksiyon içerisinde de yapabiliriz.
        use std::cmp::min;
        // min ve max fonksiyonları, x,y çiftlerinden sırasıyla en küçük ve en büyük olanları verir.
        for y in min(y1, y2)..=max(y1, y2) {
            // Eğer haritada bu x,y değerleri Duvar işareti için müsaitse de,
            if let Some(index) = self.map.try_map_to_index(Point::new(x, y)) {
                // ilgili indisteki nokta Floor olarak belirlenir.
                self.map.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    // Bu da yatay koridor çizmek için kullanılan fonksiyondur.
    fn build_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        // Kullanılacak modüle bildirimlerini buradaki gibi fonksiyon içerisinde de yapabiliriz.
        use std::cmp::min;
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = self.map.try_map_to_index(Point::new(x, y)) {
                self.map.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    // Koridorları çizmek için kullanılan fonksiyondur.
    fn build_tunnels(&mut self, gen: &mut RandomNumberGenerator) {
        // Odaları tutan vektör serisinin bir klonu alınır.
        let mut rooms = self.rooms.clone();
        // Odalar merkez x koordinatlarına göre sıralanır.
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        // ilki atlanacak şekilde tüm odaları dolaşan bir for döngüsü var.
        // i ile elde edilen indis değerini önceki odayı tespit ederken kullanıyoruz. Bu sebepten
        // skip(1) ile başladık.
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if gen.range(0, 2) == 1 {
                self.build_horizontal_tunnel(prev.x, new.x, prev.y);
                self.build_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.build_vertical_tunnel(prev.y, new.y, prev.x);
                self.build_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}
