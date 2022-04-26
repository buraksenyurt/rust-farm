use crate::constant::*;

mod constant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fahrenheit_to_celsius_test() {
        let value = 77.9;
        let expected = 25.500002;
        let actual = fahrenheit_to_celcius(value);
        assert_eq!(expected, actual);

        let value = 32.0;
        let expected = 0.0;
        let actual = fahrenheit_to_celcius(value);
        assert_eq!(expected, actual);
    }

    #[test]
    fn light_time_test() {
        let distance = 376_291_900;
        let expected = 1.2551748;
        let actual = light_time(distance);
        assert_eq!(expected, actual);

        let distance = 299_792_458;
        let expected = 1.0;
        let actual = light_time(distance);
        assert_eq!(expected, actual);
    }

    #[test]
    fn moose_body_mass_test() {
        let latitude = 60.5;
        let expected = 183.59149;
        let actual = moose_body_mass(latitude);
        assert_eq!(expected, actual);
    }

    #[test]
    fn nand_test() {
        let actual = nand(Input::Zero, Input::Zero);
        assert_eq!(actual, Input::One);

        let actual = nand(Input::Zero, Input::One);
        assert_eq!(actual, Input::One);

        let actual = nand(Input::One, Input::Zero);
        assert_eq!(actual, Input::One);

        let actual = nand(Input::One, Input::One);
        assert_eq!(actual, Input::Zero);
    }

    #[test]
    pub fn compound_interest_test() {
        let expected = 5427.4395;
        let actual = compound_interest(1000, 0.07, 25);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn wind_chill_test() {
        let expected = -39.053047;
        let actual = wind_chill(-25.0, 30.0);
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn flight_paths_test() {
        let point1 = Point::new(46.283, 86.667);
        let point2 = Point::new(-48.877, -123.393);
        let expected_distance = 17760.057;
        let actual = haversine_distance(point1, point2);
        assert_eq!(expected_distance, actual);
    }

    #[test]
    pub fn almost_pi_test() {
        let pi_number = almost_pi(5);
        assert_eq!(pi_number, 3.1415926454603365);

        let pi_number = almost_pi(1000000);
        assert_eq!(pi_number, 3.141592653589793);
    }
}

/// Sıcaklık değerini Fahrenheit'tan Santigrat'a çevirir
pub fn fahrenheit_to_celcius(f: f32) -> f32 {
    (5.0 / 9.0) * (f - 32.0)
}

/// Mesafeye göre ışığın ne kadar sürede ulaşacağını hesaplar
pub fn light_time(distance: u32) -> f32 {
    distance as f32 / C
}

/// Su seviyesinden yüksekliğe göre geyiğin tahmini iriliğini hesaplar
pub fn moose_body_mass(latitude: f32) -> f32 {
    (A * latitude) + B
}

/// Non And fonksiyonu. And kapısının değili.
/// 0 NAND 0 = 1
/// 0 NAND 1 = 1
/// 1 NAND 0 = 1
/// 1 NAND 1 = 0
pub fn nand(a: Input, b: Input) -> Input {
    if a == Input::One && b == Input::One {
        Input::Zero
    } else {
        Input::One
    }
}

/// NAND fonksiyonu için kullanılan veri modeli
#[derive(PartialEq, Debug)]
pub enum Input {
    Zero = 0,
    One = 1,
}

/// Bileşik faize göre belli yıl sonra yapılacak ödemeyi bulur.
pub fn compound_interest(amount: i32, rate: f32, year: u8) -> f32 {
    amount as f32 * ((1.0 + rate as f32).powf(year as f32))
}

/// Rüzgarın hızına göre hissedilen asıl soğukluk derecesini bulur.
pub fn wind_chill(temeprature: f32, wind_speed: f32) -> f32 {
    let w = wind_speed.powf(WIND_CHILL_5);
    WIND_CHILL_1 + (WIND_CHILL_2 * temeprature) - (WIND_CHILL_3 * w)
        + (WIND_CHILL_4 * temeprature * w)
}

/// Dünya üstündeki iki nokta arasındaki uçuş mesafesini Haversine Distance formülüne göre hesaplayan fonksiyon.
pub fn haversine_distance(a: Point, b: Point) -> f32 {
    let l1 = a.to_radians();
    let l2 = b.to_radians();
    let sin1 = f32::sin((l2.latitude - l1.latitude) / 2.0);
    let sin2 = f32::sin((l2.longtitude - l1.longtitude) / 2.0);
    let part1 = sin1.powf(2.0) + (f32::cos(l1.latitude) * f32::cos(l2.latitude) * sin2.powf(2.0));
    let part2 = part1.sqrt().asin();
    let part3 = 2.0 * R;
    part3 * part2
}

pub struct Point {
    pub latitude: f32,
    pub longtitude: f32,
}

impl Point {
    pub fn new(latitude: f32, longtitude: f32) -> Self {
        Self {
            latitude,
            longtitude,
        }
    }
    pub fn to_radians(&self) -> Self {
        Point {
            latitude: self.latitude.to_radians(),
            longtitude: self.longtitude.to_radians(),
        }
    }
}

/// Belirtilen toplam sayısına göre PI değerini hesaplar.
/// Büyük n değeri verilmesi halinde PI değeri daha isabetli bulunur.
/// Aşağıdaki yöntem Bailey, Adamchik ve Wagon'un ki.
pub fn almost_pi(try_count: u32) -> f64 {
    let mut sum: f64 = 0.0;
    for i in 0..try_count {
        let n = i as f64;
        let sum2 = (1.0 / 16.0) as f64;
        sum += ((4.0 / (8.0 * n + 1.0))
            - (2.0 / (8.0 * n + 4.0))
            - (1.0 / (8.0 * n + 5.0))
            - (1.0 / (8.0 * n + 6.0)))
            * sum2.powi(i as i32)
    }
    sum
}
