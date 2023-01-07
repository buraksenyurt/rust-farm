use crate::io::parser::parse_line;
use crate::transport::http::{HttpParseError, HttpRequest};
use std::fs::read_to_string;

pub fn read_file(file: &str) -> Vec<Result<HttpRequest, HttpParseError>> {
    let content = read_to_string(file).unwrap();
    content.split('\n').map(parse_line).collect()
}
