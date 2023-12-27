/*
   Çalışmadaki amaç aşağıdaki gibi ifade için AST çıkartacak kodu yazmak.
   AST; Abstract Syntax Tree
   SQL'deki select sorgusu benzeri ifadenin bir tokenize, parse mekanizmalarını ele alıyoruz.
*/

#[derive(Debug, PartialEq)]
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
    Compare(Compare),
}

#[derive(Debug, PartialEq)]
pub enum Compare {
    Eq,
    Gt,
    Lt,
    Neq,
    GtEq,
    LtEq,
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

// Bu fonksiyon ile de gelen token dizisini parse ediyoruz
pub fn parse(tokens: &[Token]) -> Result<TakeCommand, String> {
    let mut fields = Vec::new();
    let mut source = String::new();
    // Eğer boş bir vektör söz konusu ise Err dönülür
    if tokens.is_empty() {
        return Err("Unexpected command expression".to_string());
    }

    let mut token_iter = tokens.iter().peekable();
    match token_iter.next() {
        Some(Token::Take) => (),
        _ => return Err("Expression must start with take command".to_string()),
    }

    // tüm tokenları dolaşacak bir döngü açılır
    // bu döngü peekable iterasyon üzerinden ilerler
    while let Some(token) = token_iter.next() {
        // O anki token'ın ne olduğuna bakılır
        match token {
            // Bir tanımlayıcı ise ve bir field yakalanmışsa
            Token::Identifier(name) => {
                // field listeye eklenir
                fields.push(name.clone());
            }
            // , veya boşluk gibi comma iterasyondan devam edilir
            Token::Comma => continue,
            // From ile karşılaşılmışsa
            Token::From => {
                // Yine source adı yakalanan kadar iterasyonda hareket edilir
                if let Some(Token::Identifier(name)) = token_iter.next() {
                    source = name.clone();
                    break;
                } else {
                    // buraya Source name yakalanamadıysa gelinir
                    return Err("Expected SOURCE NAME after TAKE".to_string());
                }
            }
            // Diğer koşullar dışında hata verilir
            _ => return Err("Unexpected token in fields".to_string()),
        }
    }

    // Hiç bir field veya source olmaması da bir hatadır. Err ile cezalandırılır
    if fields.is_empty() || source.is_empty() {
        return Err("Unapplicable command expression".to_string());
    }

    Ok(TakeCommand { fields, source })
}

// Bu fonksiyon gelen metin diliminin hangi Token enum değerine denk geldiğini bulmak için kullanılır
pub fn map_token(input: &str) -> Token {
    match input.to_lowercase().as_str() {
        "take" => Token::Take,
        "from" => Token::From,
        "find" => Token::Find,
        "eq" => Token::Compare(Compare::Eq),
        "gt" => Token::Compare(Compare::Gt),
        "lt" => Token::Compare(Compare::Lt),
        "neq" => Token::Compare(Compare::Neq),
        "lteq" => Token::Compare(Compare::LtEq),
        "gteq" => Token::Compare(Compare::GtEq),
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
            Token::Compare(Compare::Eq),
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

    #[test]
    fn empty_token_vec_raise_an_error_test() {
        let tokens = vec![];
        let actual = parse(&tokens);
        assert!(actual.is_err());
    }

    #[test]
    fn valid_expression_can_parse_test() {
        let expression = "take id,title,unit_price from products";
        let tokens = tokenize(expression);
        let actual = parse(&tokens);
        let expected = TakeCommand {
            source: "products".to_string(),
            fields: vec![
                "id".to_string(),
                "title".to_string(),
                "unit_price".to_string(),
            ],
        };
        assert!(actual.is_ok());
        assert_eq!(actual, Ok(expected));
    }
}
