use crate::model::prelude::*;
use crate::token::prelude::*;

pub struct ClassToken;

impl Tokenizer for ClassToken {
    fn tokenize(code: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let lines = code.lines();
        for line in lines {
            let line = line.trim();
            if line.contains("class") {
                tokens.push(line.to_owned());
            }
        }
        tokens
    }
}

impl MultiParser for ClassToken {
    fn parse(tokens: &[String]) -> Result<Class, ()>
    where
        Self: Sized,
    {
        let mut name = String::new();
        for token in tokens {
            if let Some(class_token) = token.strip_prefix("public class ") {
                name = class_token
                    .trim_end_matches(|c| c == ' ' || c == '{' || c == '}')
                    .to_owned();
            }
        }
        Ok(Class {
            name,
            properties: vec![],
            fields: vec![],
            methods: vec![],
        })
    }
    type Output = Class;
}
