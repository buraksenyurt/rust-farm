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
           Bir döngü ile bu karakterin ne olduğuna bir bakalım.
           Aritmetik operatörleri, satır sonunu bulmamız kolay.
        */
        loop {
            match next_char {
                Some('+') => Some(Token::Add),
                Some('-') => Some(Token::Substract),
                Some('/') => Some(Token::Divide),
                Some('*') => Some(Token::Multiply),
                Some('^') => Some(Token::Caret),
                Some('(') => Some(Token::LeftBracket),
                Some(')') => Some(Token::RightBracket),
                None => Some(Token::EOF),
                _ => None,
            }
        }
    }
}
