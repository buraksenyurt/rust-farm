pub trait Tokenizer {
    fn tokenize(code: &str) -> Vec<String>;
    fn parse(tokens: &[String]) -> Result<Self::Output, ()>
    where
        Self: Sized;
    type Output;
}
