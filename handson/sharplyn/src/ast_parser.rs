use crate::ast_model::{Class, Unit};
use crate::ast_token::{ClassToken, NamespaceToken, Tokenizer};

pub fn parse_code(code: &str) -> Result<Unit, ()> {
    let namespace_tokens = NamespaceToken::tokenize(code);
    let namespace = NamespaceToken::parse(&namespace_tokens).expect("Can't find/read namespace");

    let class_tokens = ClassToken::tokenize(code);
    let classes: Vec<Class> = class_tokens
        .iter()
        .filter_map(|token| ClassToken::parse(&[token.clone()]).ok())
        .collect();

    Ok(Unit { namespace, classes })
}
