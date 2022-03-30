#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_string_version_works_for_some_numbers() {
        let number = 3;
        assert_eq!(fizz_buzz_v1(number), "Fizz");
        let number = 5;
        assert_eq!(fizz_buzz_v1(number), "Buzz");
        let number = 15;
        assert_eq!(fizz_buzz_v1(number), "FizzBuzz");
        let number = 4;
        assert_eq!(fizz_buzz_v1(number), "4");
    }
}

pub fn fizz_buzz_v1(i: i32) -> String {
    if i % 15 == 0 {
        "FizzBuzz".to_string()
    } else if i % 3 == 0 {
        "Fizz".to_string()
    } else if i % 5 == 0 {
        "Buzz".to_string()
    } else {
        i.to_string()
    }
}
