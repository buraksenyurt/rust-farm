/*
   Esasında parser modülü lexer'dan yararlanıp ve ifadenin söz dizimi ağacını çıkarmak için kullanılır.
*/

use crate::parser::lexer::Lexer;
use crate::parser::node::Node;
use crate::parser::node::Node::Number;
use crate::parser::token::{Precedence, Token};

/*
   Veri modelimizde token'ları işleyen lexer ve güncel token'ın tutulduğu bir alan yer alıyor.
*/
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    // Yeni bir Parser örneği oluşturmak için kullanılan fonksiyon.
    pub fn new(expression: &'a str) -> Self {
        let mut lexer = Lexer::new(expression);
        let current_token = lexer.next().unwrap();
        Parser {
            lexer,
            current_token,
        }
    }
    // Parse işlemini gerçekleştirip root node'u döndürmesini planladığımız fonksiyon
    pub fn parse(&mut self) -> Node {
        self.generate_tree(Precedence::Zero)
    }

    // ağaç yapısını çıkartacak fonksiyon.
    fn generate_tree(&mut self, precedence: Precedence) -> Node {
        todo!()
    }

    // Takip eden token'ı çeken fonksiyon. Sonraki token'ı çekip güncel token alanını da bununla değiştirir.
    fn get_next_token(&mut self) {
        let next_token = self.lexer.next();
        self.current_token = next_token.unwrap();
    }

    fn check_bracket(&mut self, expected: Token) -> bool {
        todo!()
    }

    fn parse_number(&mut self) -> Node {
        todo!()
    }
}

// Bunu Node::from(self.current_token()) gibi kullanabiliriz belki.
impl From<Token> for Node {
    fn from(token: Token) -> Self {
        todo!()
    }
}
