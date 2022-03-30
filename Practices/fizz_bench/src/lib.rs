use std::fmt::{Display, Formatter};

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

    #[test]
    fn should_number_convertable_to_word_enum() {
        let number = 3;
        assert_eq!(fizz_buzz_v2(number), Word::Fizz);
        let number = 5;
        assert_eq!(fizz_buzz_v2(number), Word::Buzz);
        let number = 15;
        assert_eq!(fizz_buzz_v2(number), Word::FizzBuzz);
        let number = 4;
        assert_eq!(fizz_buzz_v2(number), Word::Number(4));
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

#[derive(Debug, PartialEq)]
pub enum Word {
    /// 3 ile tam bölünenler
    Fizz,
    /// 5 ile tam bölünenler
    Buzz,
    /// 15 ile tam bölünenler
    FizzBuzz,
    /// Diğer sayılar
    Number(i32),
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Word::Fizz => f.write_str("Fizz"),
            Word::Buzz => f.write_str("Buzz"),
            Word::FizzBuzz => f.write_str("FizzBuzz"),
            Word::Number(n) => write!(f, "{}", n),
        }
    }
}

pub fn fizz_buzz_v2(number: i32) -> Word {
    if number % 15 == 0 {
        Word::FizzBuzz
    } else if number % 3 == 0 {
        Word::Fizz
    } else if number % 5 == 0 {
        Word::Buzz
    } else {
        Word::Number(number)
    }
}
