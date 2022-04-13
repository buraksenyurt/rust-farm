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
}

pub const C: f32 = 299_792_458.0;
pub const A: f32 = 2.757;
pub const B: f32 = 16.793;

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
    let w = wind_speed.powf(0.16);
    13.12 + (0.6215 * temeprature) - (11.37 * w) + (0.3956 * temeprature * w)
}
