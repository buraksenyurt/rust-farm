use crate::prelude::*;
// Constant'ları tanımlarken başka Constant'lardan oluşturabiliriz.
// Sistemin çalıştığı işlemci mimarisine göre usize 64bit veya 32bit olur.
// Bu sabit ile aslında oyun sahasındaki haritanın fayanslarının sayısını tutuyoruz.
// Saha genişliği veya yüksekliği değişirse bu sabit değer de otomatik olarak değişecektir.
const NUMBER_OF_TILES: usize = { SCHENE_WIDTH * SCHENE_HEIGHT } as usize;

// Haritadaki hücrelerin tipleri. Duvar olabilir veya bir zemindir.
#[derive(Copy, Clone)]
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
    // Ekran koordinatlarını dolaşırken, x,y karşılığı olan indis değerini bulur,
    // bu değerin karşılığı olan vector alanının tipine bakarak bir şey çizer.
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCHENE_HEIGHT {
            for x in 0..SCHENE_WIDTH {
                let index = map_to_index(x, y);
                match self.tiles[index] {
                    TileType::Wall => ctx.set(x, y, WHITE, BLACK, to_cp437('X')),
                    TileType::Floor => ctx.set(x, y, RED, BLACK, to_cp437('_')),
                }
            }
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
