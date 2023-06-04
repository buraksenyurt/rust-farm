use crate::model::prelude::*;
use crate::token::prelude::*;

pub fn parse_code(code: &str) -> Result<Unit, ()> {
    let using_tokens = UsingToken::tokenize(code);
    let usings: Vec<Using> = using_tokens
        .iter()
        .filter_map(|token| UsingToken::parse(&[token.clone()]).ok())
        .collect();

    let namespace_tokens = NamespaceToken::tokenize(code);
    let namespaces: Vec<Namespace> = namespace_tokens
        .iter()
        .filter_map(|token| NamespaceToken::parse(&token.clone()).ok())
        .collect();

    let class_tokens = ClassToken::tokenize(code);
    let classes: Vec<Class> = class_tokens
        .iter()
        .filter_map(|token| ClassToken::parse(&[token.clone()]).ok())
        .collect();

    Ok(Unit {
        namespaces,
        classes,
        usings,
    })
}
