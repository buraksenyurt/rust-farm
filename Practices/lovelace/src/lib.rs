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
}

pub const C: f32 = 299_792_458.0;
pub const A: f32 = 2.757;
pub const B: f32 = 16.793;

pub fn fahrenheit_to_celcius(f: f32) -> f32 {
    (5.0 / 9.0) * (f - 32.0)
}

pub fn light_time(distance: u32) -> f32 {
    distance as f32 / C
}

pub fn moose_body_mass(latitude: f32) -> f32 {
    (A * latitude) + B
}
