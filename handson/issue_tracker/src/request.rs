use std::str::FromStr;

#[derive(Debug,PartialEq)]
pub struct Request {
    pub method: RequestMethod,
    pub protocol: String,
    pub route: String,
}

impl Request {
    pub fn new(method: RequestMethod, protocol: String, route: String) -> Self {
        Self {
            method,
            protocol,
            route,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    DELETE,
    GET,
    POST,
    PUT,
}

#[derive(Debug)]
pub enum RequestConvertError {
    Invalid,
    InvalidHttpMethod,
}

impl FromStr for Request {
    type Err = RequestConvertError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        if line_parts.len() != 3 {
            return Err(RequestConvertError::Invalid);
        }
        let method = match line_parts[0] {
            "GET" => RequestMethod::GET,
            "POST" => RequestMethod::POST,
            "PUT" => RequestMethod::PUT,
            "DELETE" => RequestMethod::DELETE,
            _ => return Err(RequestConvertError::InvalidHttpMethod),
        };

        Ok(Request::new(
            method,
            line_parts[2].to_string(),
            line_parts[1].to_string(),
        ))
    }
}
