/*
   Çalışmadaki amaç aşağıdaki gibi ifade için AST çıkartacak kodu yazmak.
   AST; Abstract Syntax Tree
   SQL'deki select sorgusu benzeri ifadenin bir tokenize, parse mekanizmalarını ele alıyoruz.
*/

pub struct TakeCommand {
    fields: Vec<String>,
    source: String,
}

pub enum Query {
    Take(TakeCommand),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Take,
    From,
    Find,
    Identifier(String),
    Comma,
    Equality,
}

// İfade içerisinde token'ları çekmek için kullanacağımız metot
pub fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut cur_token = String::new();

    // İfade içerisindeki tüm karakterler Peekable türünden bir vektöre alınır
    let mut char_list = expression.chars().peekable();
    // Vektörde okuma yapılabildiği sürece devam edecek bir döngü başlatılır
    while let Some(&c) = char_list.peek() {
        // o anki karakter için bir match ifadesi açılır
        // son ifadeden okursak daha kolay olur.
        // Herhangibir boşluk veya virgül ile karşılaşılmassa karakterler cur_token isimli string'e eklenirler
        // Böylece take, from, where gibi metinsel ifadelere ulaşılabilir.
        match c {
            // boşluk veya , varsa o ana kadar cur_token içinde birikmiş metinsel ifade hangi token'a karşılık geliyor
            // diye ilgili fonksiyon çağrılabilir.
            ' ' | ',' if !cur_token.is_empty() => {
                tokens.push(map_token(&cur_token));
                cur_token.clear();
            }
            ' ' => {}
            ',' => tokens.push(Token::Comma),
            _ => cur_token.push(c),
        }
        char_list.next();
    }

    if !cur_token.is_empty() {
        tokens.push(map_token(&cur_token));
    }

    tokens
}

// Bu fonksiyon gelen metin diliminin hangi Token enum değerine denk geldiğini bulmak için kullanılır
pub fn map_token(input: &str) -> Token {
    match input.to_lowercase().as_str() {
        "take" => Token::Take,
        "from" => Token::From,
        "find" => Token::Find,
        "eq" => Token::Equality,
        _ => Token::Identifier(input.to_string()), // field adlarını yakalarız
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_a_simple_query_test() {
        let expression = "take id,title,unit_price from products";
        let actual = tokenize(expression);
        let expected = vec![
            Token::Take,
            Token::Identifier("id".to_string()),
            Token::Identifier("title".to_string()),
            Token::Identifier("unit_price".to_string()),
            Token::From,
            Token::Identifier("products".to_string()),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_a_query_with_find_test() {
        let expression = "take id,title,unit_price from products find category_id eq 10";
        let actual = tokenize(expression);
        let expected = vec![
            Token::Take,
            Token::Identifier("id".to_string()),
            Token::Identifier("title".to_string()),
            Token::Identifier("unit_price".to_string()),
            Token::From,
            Token::Identifier("products".to_string()),
            Token::Find,
            Token::Identifier("category_id".to_string()),
            Token::Equality,
            Token::Identifier("10".to_string()),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_an_empty_expression_query_test() {
        let expression = "";
        let actual = tokenize(expression);
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_a_wrong_query_test() {
        let expression = "bla bla bla";
        let actual = tokenize(expression);
        let expected = vec![
            Token::Identifier("bla".to_string()),
            Token::Identifier("bla".to_string()),
            Token::Identifier("bla".to_string()),
        ];
        assert_eq!(actual, expected);
    }
}
