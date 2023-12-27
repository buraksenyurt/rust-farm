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
    Take(TakeCommand)
}

pub enum Token {
    Take,
    From,
    Identifier(String),
    Comma
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
