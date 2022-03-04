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
