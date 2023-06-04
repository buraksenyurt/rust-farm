use crate::model::prelude::*;
use crate::token::prelude::*;

pub struct UsingToken;

impl Tokenizer for UsingToken {
    fn tokenize(code: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let lines = code.lines();
        for line in lines {
            let line = line.trim();
            if line.strip_prefix("using").is_some() {
                tokens.push(line.to_owned());
            }
        }
        tokens
    }

    fn parse(tokens: &[String]) -> Result<Using, ()>
    where
        Self: Sized,
    {
        let mut name = String::new();
        for token in tokens {
            if let Some(using_token) = token.strip_prefix("using ") {
                name = using_token.trim_end_matches(|c| c == ';').to_owned();
            }
        }
        Ok(Using { name })
    }
    type Output = Using;
}
