use crate::model::prelude::*;
use crate::token::prelude::*;
use regex::Regex;

pub struct UsingToken;

impl Tokenizer for UsingToken {
    fn tokenize(code: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let lines = code.lines();
        for line in lines {
            let line = line.trim();
            if Regex::new(r"using").unwrap().is_match(line) {
                tokens.push(line.to_owned());
            }
        }
        tokens
    }
}

impl MultiParser for UsingToken {
    fn parse(tokens: &[String]) -> Result<Using, ()>
    where
        Self: Sized,
    {
        let mut name = String::new();
        for token in tokens {
            if let Some(t) = Regex::new(r"using\s+([^;]+)").unwrap().captures(token) {
                name = t.get(1).unwrap().as_str().to_owned();
            }
        }
        Ok(Using { name })
    }
    type Output = Using;
}
