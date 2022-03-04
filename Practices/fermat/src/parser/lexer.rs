use crate::parser::token::Token;
use std::iter::Peekable;
use std::str::Chars;

/*
   Lexer veya Tokenizer için kullanacağımız veri modeli.

   Girilen metni karakter bazında okumak gerekiyor.
   Lakin bir sonraki karakterin ne olduğunu bilmemiz önemli.
   Nitekim 21 ile 1+ gibi ifadelerini düşündüğümüzde 21'i bir sayı olarak düşünmek,
   1+ yı ise 1 ve + şeklinde iki farklı token olarak okumak gerekiyor.

   Peekable güncel ve bir sonraki karakteri opsiyonel olarak almamızı sağladığından
   yukarıdaki ayrıştırmada epey işimize yarayabilir.
*/
pub struct Lexer<'a> {
    expression: Peekable<Chars<'a>>,
}

/*
   Bir Lexer değişkenini new ile oluşturabiliriz.
   Parametre olarak bir string dilimini alır.
   Tabii bu bir referans olacağından ve Lexer için yaşam ömrü açıkça belirtildiğinden,
   burada da 'a kullanmalıyız.
   new fonksiyonu gelen ifadeyi kullanarak veri modelini oluşturur.

*/
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            expression: input.chars().peekable(),
        }
    }
}

/*
   ifade ile gelen karakterlerde ileri yönlü hareket etmenin en kolay yolu,
   iterator desenini uygulamak olacak.
   Bu amaçla built-in gelen Iterator trait'ini kullanabiliriz.

   Trait'ten uyguladığımız next fonksiyonu ile token'ları tek tek elde edebiliriz.
*/
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Sonraki karakteri çekelim
        let next_char = self.expression.next();
        /*
           Aritmetik operatörleri ve satır sonunu bulmamız kolay.

           Bir rakam olup olmadığını '0'..='9' ile kontrol edebiliriz.

           next_char eğer None dönüyorsa zaten ifadenin satır sonuna gelmişizdir.
        */

        match next_char {
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Substract),
            Some('/') => Some(Token::Divide),
            Some('*') => Some(Token::Multiply),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftBracket),
            Some(')') => Some(Token::RightBracket),
            Some('0'..='9') => {
                // O anki karakter 0 veya 9 ise bu blok çalışacak.
                // Önce takip eden karakter olup olmadığına bakarak number değişkenini hazırlayalım
                let mut number = next_char?.to_string();
                // takip eden karakter olduğu sürece çalışacak bir döngü açıyoruz
                while let Some(next_char) = self.expression.peek() {
                    /*
                       sayısal bir değer veya , ise onu number değişkenine ilave edip devam edelim
                       sol parantez ise başka bir token başladığını anlamış oluruz. O nedenle None dönebiliriz.
                       Hiçbir koşula uymuyorsa da break ile döngüden çıkartırız ve number'ın son halini elde etmiş oluruz.
                    */
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expression.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }
                Some(Token::Number(number.parse::<f64>().unwrap()))
            }
            None => Some(Token::EndOfExpression),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn should_arithmetic_operators_returns_valid_tokens() {
        let mut lexer = Lexer::new("+");
        assert_eq!(lexer.next().unwrap(), Token::Add);
        let mut lexer = Lexer::new("*");
        assert_eq!(lexer.next().unwrap(), Token::Multiply);
        let mut lexer = Lexer::new("/");
        assert_eq!(lexer.next().unwrap(), Token::Divide);
        let mut lexer = Lexer::new("-");
        assert_eq!(lexer.next().unwrap(), Token::Substract);
    }

    #[test]
    fn should_numbers_returns_number_tokens() {
        let mut lexer = Lexer::new("3.141592653589793");
        assert_eq!(lexer.next().unwrap(), Token::Number(PI));

        let mut lexer = Lexer::new("23");
        assert_eq!(lexer.next().unwrap(), Token::Number(23_f64));
    }
}
