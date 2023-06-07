use std::str::FromStr;
use std::string::ParseError;

pub struct Request {
    pub method: Option<String>,
    pub path: Option<String>,
    pub protocol: Option<String>,
}

impl FromStr for Request {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut tokens = input.split_whitespace();
        let method = match tokens.next() {
            Some(t) => Some(String::from(t)),
            None => None,
        };
        let path = match tokens.next() {
            Some(t) => Some(String::from(t)),
            None => None,
        };
        let protocol = match tokens.next() {
            Some(t) => Some(String::from(t)),
            None => None,
        };
        Ok(Self {
            method,
            path,
            protocol,
        })
    }
}
