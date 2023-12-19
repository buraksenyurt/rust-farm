/*
   enum türüne ait farklı kullanımların yer aldığı kod dosyasıdır.
*/

use std::f32::consts::PI;

// Basit bir kullanım. Yön bilgilerini tutan bir enum türü
enum Direction {
    East,
    North,
    South,
    West,
}

struct Address {
    pub line_1: String,
    pub line_2: String,
    pub city: String,
    pub postal_code: String,
}
// Lokasyon bilgisinin farklı varyasyonlarla ele alınması
enum Location {
    Nowhere,
    Address(Address),
    Coordinates { latitude: f32, longitude: f32 },
}

// Farklı türleri barındıran ortak amaçta birleşebilecek varyasyonlar(variants)
enum Message {
    Exit,
    Move { x: i32, y: i32 },
    Log(String),
}

// Basit bir polimorfizim uyarlaması
enum Shape {
    Rectangle { w: f32, h: f32 },
    Circle { r: f32 },
    Square { w: f32 },
}
impl Shape {
    pub fn calc_area(&self) -> f32 {
        match self {
            Shape::Rectangle { w, h } => w * h,
            Shape::Circle { r } => PI * r * r,
            Shape::Square { w } => w * w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_tests() {
        let shapes = vec![
            Shape::Circle { r: 10. },
            Shape::Square { w: 10. },
            Shape::Rectangle { w: 5., h: 10. },
        ];
        assert_eq!(shapes[0].calc_area(), 314.15927);
        assert_eq!(shapes[1].calc_area(), 100.);
        assert_eq!(shapes[2].calc_area(), 50.);
    }
}
