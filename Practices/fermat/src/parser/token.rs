/*
   Girilen ifade parse edildiğinde neyin ne olduğunu tutacağımız veri modeli.
   Bunu enum sabiti olarak tasarlamak son derece mantıklı. Bir toplama operatörü ile
   karşılaştıysak Add, üs alma operatörü ise Caret, ifade sonuna geldiysek EOF,
   bir sayı söz konusu ise Number(f64) vs şeklinde atamalar yapabiliriz.
*/

#[derive(Debug, PartialEq)]
pub enum Token {
    Add,
    Substract,
    Multiply,
    Divide,
    Caret,
    LeftBracket,
    RightBracket,
    Number(f64),
    EndOfExpression,
}

// Bu enum sabiti ile işlem önceliklerini en düşük olandan en önemli olana doğru sırlamaktayız.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Zero,
    AddSub,
    MulDiv,
    Power,
    Negative,
}

/*
   İşlem önceliğini bulmak için aşağıdaki fonksiyondan yararlanabiliriz.
   Token'ının ne olduğuna bakarak geriye önceliklendirme enum sabitindeki ilgili değeri döndürmekte.
*/
impl Token {
    pub fn get_precedence(&self) -> Precedence {
        use self::Precedence::*;
        use self::Token::*;

        match self {
            Add | Substract => AddSub,
            Multiply | Divide => MulDiv,
            Caret => Power,
            _ => Zero,
        }
    }
}
