use crate::model::prelude::{Parameter, SharpType};
use crate::token::prelude::{MultiParser, Tokenizer};
use regex::Regex;

pub struct ParameterToken {
    pub name: String,
    pub data_type: String,
}

impl Tokenizer for ParameterToken {
    fn tokenize(code: &str) -> Vec<String> {
        let mut tokens = Vec::new();

        let parameter_regex =
            Regex::new(r"^\s*([a-zA-Z_][a-zA-Z0-9_.<>]*)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*$").unwrap();

        for line in code.lines() {
            let line = line.trim();
            if parameter_regex.is_match(line) {
                tokens.push(line.to_owned());
            }
        }
        tokens
    }
}

impl MultiParser for ParameterToken {
    fn parse(tokens: &[String]) -> Result<Parameter, ()> {
        let mut name = String::new();
        let mut data_type = String::new();

        let parameter_regex =
            Regex::new(r"^\s*([a-zA-Z_][a-zA-Z0-9_.<>]*)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*$").unwrap();

        if let Some(token) = tokens.first() {
            if let Some(captures) = parameter_regex.captures(token) {
                name = captures[2].to_owned();
                data_type = captures[1].to_owned();
            }
        }
        let type_r = match data_type.as_str() {
            "byte" => SharpType::Byte,
            "short" => SharpType::Short,
            "int" => SharpType::Int,
            "float" => SharpType::Float,
            "decimal" => SharpType::Decimal,
            "string" => SharpType::String,
            _ => SharpType::Object,
        };
        Ok(Parameter {
            name,
            data_type: type_r,
        })
    }

    type Output = Parameter;
}
