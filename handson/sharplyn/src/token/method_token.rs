use crate::model::prelude::*;
use crate::token::prelude::*;
use regex::Regex;

pub struct MethodToken {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<String>,
    pub visibility: String,
}

impl BodyTokenizer for MethodToken {
    type Token = MethodToken;
    type Output = Method;

    fn tokenize(code: &str) -> Vec<Self::Token> {
        //println!("Gelen bilgi {}", code);
        let mut tokens = Vec::new();
        let method_regex = Regex::new(r"(?m)^\s*(public|internal|protected|private)?\s*(?:static)?\s*(?:async)?\s*(?:unsafe)?\s*(?:extern)?\s*([\w<>]+)\s+(\w+)\s*\((.*?)\)\s*[{;]").unwrap();

        for captures in method_regex.captures_iter(code) {
            //println!("Bir metot deseni yakalandı");
            let visibility = captures
                .get(1)
                .map(|m| {
                    let visibility = m.as_str().trim();
                    if visibility.is_empty() {
                        "internal"
                    } else {
                        visibility
                    }
                })
                .unwrap()
                .to_owned();
            let return_type = captures
                .get(2)
                .map(|m| m.as_str().trim().to_owned())
                .unwrap();
            let name = captures.get(3).map(|m| m.as_str().trim().to_owned());
            let parameters_line = captures.get(4).map(|m| m.as_str().trim().to_owned());

            let mut parameters = Vec::new();
            let param_regex =
                Regex::new(r"([a-zA-Z_][a-zA-Z0-9_.<>]*)\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();

            for capture in param_regex.captures_iter(parameters_line.as_deref().unwrap_or_default())
            {
                let param_type = capture[1].to_owned();
                let param_name = capture[2].to_owned();
                parameters.push(format!("{} {}", param_type, param_name));
            }

            if let (Some(name), parameters) = (name, parameters) {
                tokens.push(MethodToken {
                    name,
                    return_type,
                    parameters,
                    visibility,
                });
            }
        }
        tokens
    }

    fn parse(token: &Self::Token) -> Result<Self::Output, ()> {
        let name = &token.name;
        let return_type = &token.return_type;
        let parameters = &token.parameters;

        let type_r = match return_type.as_str() {
            "byte" => SharpType::Byte,
            "short" => SharpType::Short,
            "int" => SharpType::Int,
            "float" => SharpType::Float,
            "decimal" => SharpType::Decimal,
            "string" => SharpType::String,
            "void" => SharpType::Void,
            _ => SharpType::Object,
        };

        let mut parsed_params = Vec::new();

        for param in parameters {
            let token = ParameterToken::tokenize(param);
            if let Ok(parameter) = ParameterToken::parse(&token) {
                parsed_params.push(parameter);
            }
        }

        Ok(Method {
            name: name.to_owned(),
            return_type: type_r,
            parameters: parsed_params,
        })
    }
}
