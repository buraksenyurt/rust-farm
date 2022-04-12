#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_77_9_fahrenheit_is_25_5_celsius_test() {
        let value = 77.9;
        let expected = 25.500002;
        let actual = fahrenheit_to_celcius(value);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_32_fahrenheit_is_0_celsius_test() {
        let value = 32.0;
        let expected = 0.0;
        let actual = fahrenheit_to_celcius(value);
        assert_eq!(expected, actual);
    }
}

pub fn fahrenheit_to_celcius(f: f32) -> f32 {
    (5.0 / 9.0) * (f - 32.0)
}
