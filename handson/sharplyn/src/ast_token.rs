use crate::ast_model::{Class, Namespace};

pub trait Tokenizer {
    fn tokenize(code: &str) -> Vec<String>;
    fn parse(tokens: &[String]) -> Result<Self::Output, ()>
    where
        Self: Sized;
    type Output;
}

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

    fn parse(tokens: &[String]) -> Result<Namespace, ()> {
        let mut name = String::new();
        for token in tokens {
            if let Some(namespace_token) = token.strip_prefix("public namespace ") {
                name = namespace_token
                    .trim_end_matches(|c| c == ' ' || c == '{')
                    .to_owned();
            }
        }
        Ok(Namespace {
            name,
            classes: vec![],
        })
    }
    type Output = Namespace;
}

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

    fn parse(tokens: &[String]) -> Result<Class, ()>
    where
        Self: Sized,
    {
        let mut name = String::new();
        for token in tokens {
            if let Some(class_token) = token.strip_prefix("public class ") {
                name = class_token
                    .trim_end_matches(|c| c == ' ' || c == '{')
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
