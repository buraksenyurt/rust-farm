use std::fmt::{Display, Formatter};

// Rust açısından baktığımızda Fizz Buzz serisi için enum sabiti kullanarak tüm olasılıkları
// ekstra allocation olmadan kullanabiliriz.
// Fizz Buzz için olası çıktılar normalde aşağıdaki gibidir.
/// Fizz Buzz durumunu taşıyan veri yapısıdır
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

// Enum bilgisini herhangi bir formatter'a yazılmasını sağlıyoruz.
// Örneğin println! makrosundaki {} ifadesinde de kolayca kullanabiliriz.
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

// Fizz Buzz kontrolü yapan fonksiyonumuz artık String veya &str yerine
// Word yapısından bir enum değeri döndürecek
pub fn get_fizz_buzz(number: i32) -> Word {
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

#[cfg(test)]
mod test {
    use crate::fermat::{get_fizz_buzz, Word};

    #[test]
    fn should_number_convertable_to_word_enum() {
        let number = 3;
        assert_eq!(get_fizz_buzz(number), Word::Fizz);
        let number = 5;
        assert_eq!(get_fizz_buzz(number), Word::Buzz);
        let number = 15;
        assert_eq!(get_fizz_buzz(number), Word::FizzBuzz);
        let number = 4;
        assert_eq!(get_fizz_buzz(number), Word::Number(4));
    }
}
