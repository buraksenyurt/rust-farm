use crate::model::prelude::*;
use crate::token::prelude::*;
use regex::Regex;

pub struct NamespaceToken;

impl Tokenizer for NamespaceToken {
    fn tokenize(code: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let lines = code.lines();
        for line in lines {
            let line = line.trim();
            if Regex::new(r"namespace").unwrap().is_match(line) {
                tokens.push(line.to_owned());
            }
        }
        tokens
    }
}

impl SingleParser for NamespaceToken {
    fn parse(token: &str) -> Result<Namespace, ()> {
        let mut name = String::new();
        if let Some(t) = Regex::new(r"(public\s+)?namespace (\w+)")
            .unwrap()
            .captures(token)
        {
            name = t.get(2).unwrap().as_str().to_owned();
        }
        Ok(Namespace {
            name,
            classes: vec![],
        })
    }
    type Output = Namespace;
}
