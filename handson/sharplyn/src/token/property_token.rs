use crate::model::prelude::*;
use crate::token::prelude::*;
use crate::token::tokenizer::SingleParser;
use regex::Regex;

pub struct PropertyToken;

impl Tokenizer for PropertyToken {
    fn tokenize(code: &str) -> Vec<String> {
        let mut tokens = Vec::new();

        let lines = code.lines();
        for line in lines {
            let line = line.trim();
            if Regex::new(r"(get|set)").unwrap().is_match(line) {
                tokens.push(line.to_owned());
            }
        }
        tokens
    }
}

impl SingleParser for PropertyToken {
    fn parse(token: &str) -> Result<Property, ()> {
        let parts: Vec<&str> = token.split_whitespace().collect();
        let p_type = match parts[1] {
            "byte" => SharpType::Byte,
            "short" => SharpType::Short,
            "int" => SharpType::Int,
            "float" => SharpType::Float,
            "decimal" => SharpType::Decimal,
            "string" => SharpType::String,
            _ => SharpType::Object,
        };
        let property = Property {
            name: parts[2].to_owned(),
            p_type,
        };

        Ok(property)
    }

    type Output = Property;
}
