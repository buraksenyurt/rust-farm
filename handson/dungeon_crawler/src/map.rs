use crate::prelude::*;
// Constant'ları tanımlarken başka Constant'lardan oluşturabiliriz.
// Sistemin çalıştığı işlemci mimarisine göre usize 64bit veya 32bit olur.
// Bu sabit ile aslında oyun sahasındaki haritanın fayanslarının sayısını tutuyoruz.
// Saha genişliği veya yüksekliği değişirse bu sabit değer de otomatik olarak değişecektir.
const NUMBER_OF_TILES: usize = { SCHENE_WIDTH * SCHENE_HEIGHT } as usize;

// Haritadaki hücrelerin tipleri. Duvar olabilir veya bir zemindir.
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

// Oyun sahasını sembolize eden veri yapımız.
// Sahadaki fayansların ne olduğu bilgisini TileType türünden bir vector olarak tutmakta.
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            // Oyun sahasında ne kadar hücre varsa başlangıçta boş zemin olarak kabul ederek
            // tiles değişkeni oluşturulmakta.
            tiles: vec![TileType::Floor; NUMBER_OF_TILES],
        }
    }

    // Bu fonksiyon haritayı ekrana çizmek için kullanılır.
    // Oyuna eklenen kamerayı kullanarak kameranın baktığı alan için bir render işlemi uygular.
    // Ekran koordinatlarını dolaşırken, x,y karşılığı olan indis değerini bulur,
    // bu değerin karşılığı olan vector alanının tipine bakarak bir şey çizer.
    pub fn render(&self, ctx: &mut BTerm, visor: &Camera) {
        // main fonksiyonundaki builder'daki ilk console layer'ı kullanacağımızı söyledik.
        ctx.set_active_console(0);
        for y in visor.top_y..visor.bottom_y {
            for x in visor.left_x..visor.right_x {
                if self.is_in_bounds(Point::new(x, y)) {
                    let index = map_to_index(x, y);
                    match self.tiles[index] {
                        TileType::Wall => ctx.set(
                            x - visor.left_x,
                            y - visor.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('#'),
                        ),
                        TileType::Floor => ctx.set(
                            x - visor.left_x,
                            y - visor.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        ),
                    }
                }
            }
        }
    }

    // Bu fonksiyon ile Point ile gelen x,y koordinatlarının oyun sahası içinde olup olmadığı
    // kontrol ediliyor. Oyuncunun saha içinde olup olmadığını anlamak için eklenmiş olan yardımcı
    // bir fonksiyon.
    pub fn is_in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCHENE_WIDTH && point.y >= 0 && point.y < SCHENE_HEIGHT
    }

    // Bu fonksiyon ile de gelen x,y koordinatlarının oyun sahası içinde ve bir zemin olup
    // olmadığı kontrol ediliyor. Oyuncunun oyun sahası içinde olması ve
    // bir duvardan(TileType::Wall) geçmeye çalışıp çalışmadığını kontrol eden yardımcı fonksiyon.
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.is_in_bounds(point) && self.tiles[map_to_index(point.x, point.y)] == TileType::Floor
    }

    // Bu fonksiyon ile x,y koordinatlarının map vektöründeki bir indise dönüştürülebileceğini
    // garanti altına almış oluyoruz. Eğer x,y oyun sahası sınırları içinde değilse None dönüyor.
    // Oyun sahası içindeyse de karşılığı olan vecktor indis değerini çeken fonksiyonu kullanıyor.
    pub fn try_map_to_index(&self, point: Point) -> Option<usize> {
        if !self.is_in_bounds(point) {
            None
        } else {
            Some(map_to_index(point.x, point.y))
        }
    }
}

// Bu fonksiyon ile ekrandaki x,y koordinatının Map veri yapısının tuttuğu haritayı temsil eden
// tiles isimli vector'deki hangi indise karşılık geldiğini buluruz.
// Bu dönüştürme için üç teknik var. Y-First, X-First(Column First) ve Morton Encoding.
// Performan sorunları olursa Morton Encoding tercih edilebilir.
// Kitapta bunun için Y-First(Row-First) tekniği kullanılmış.
pub fn map_to_index(x: i32, y: i32) -> usize {
    ((y * SCHENE_WIDTH) + x) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_outside_coordinates_returns_false() {
        let m = Map::new();
        let actual = m.is_in_bounds(Point::new(81, 51));
        assert_eq!(actual, false);
    }

    #[test]
    fn should_valid_coordinates_returns_true() {
        let m = Map::new();
        let actual = m.is_in_bounds(Point::new(50, 25));
        assert_eq!(actual, true);
    }

    #[test]
    fn should_player_can_enter_tile() {
        let m = Map::new();
        let actual = m.can_enter_tile(Point::new(10, 10));
        assert_eq!(actual, true);
    }

    #[test]
    fn should_coordinates_can_convert_to_vector_index() {
        let m = Map::new();
        let actual = m.try_map_to_index(Point::new(10, 10));
        assert_eq!(actual, Some(810));
    }
}
