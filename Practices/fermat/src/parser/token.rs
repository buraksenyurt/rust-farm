/*
    Girilen ifade parse edildiğinde neyin ne olduğunu tutacağımız veri modeli.
    Bunu enum sabiti olarak tasarlamak son derece mantıklı. Bir toplama operatörü ile
    karşılaştıysak Add, üs alma operatörü ise Caret, ifade sonuna geldiysek EOF,
    bir sayı söz konusu ise Number(f64) vs şeklinde atamalar yapabiliriz.
 */
pub enum Token{
    Add,
    Substract,
    Multiply,
    Divide,
    Caret,
    LeftBracket,
    RightBracket,
    Number(f64),
    EndOfExpression
}