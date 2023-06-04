pub trait Tokenizer {
    fn tokenize(code: &str) -> Vec<String>;
}

pub trait MultiParser {
    fn parse(tokens: &[String]) -> Result<Self::Output, ()>
    where
        Self: Sized;
    type Output;
}

pub trait SingleParser {
    fn parse(tokens: &String) -> Result<Self::Output, ()>
    where
        Self: Sized;
    type Output;
}

pub trait BodyTokenizer {
    type Token;
    type Output;

    fn tokenize(code: &str) -> Vec<Self::Token>;
    fn parse(token: &Self) -> Result<Self::Output, ()>
    where
        Self: Sized;
}
