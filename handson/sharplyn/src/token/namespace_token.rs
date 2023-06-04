use crate::model::prelude::*;
use crate::token::prelude::*;

pub struct NamespaceToken;

impl Tokenizer for NamespaceToken {
    fn tokenize(code: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let lines = code.lines();
        for line in lines {
            let line = line.trim();
            if line.contains("namespace") {
                tokens.push(line.to_owned());
            }
        }
        tokens
    }
}

impl SingleParser for NamespaceToken {
    fn parse(token: &String) -> Result<Namespace, ()> {
        let mut name = String::new();
        if let Some(t) = token.strip_prefix("public namespace ") {
            name = t.trim_end_matches(|c| c == ' ' || c == '{').to_owned();
        }
        Ok(Namespace {
            name,
            classes: vec![],
        })
    }
    type Output = Namespace;
}
