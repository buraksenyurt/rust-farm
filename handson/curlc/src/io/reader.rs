use crate::io::parser::{parse_line, LineOutput};
use std::fs::read_to_string;

// pub fn read_file(file: &str) -> Vec<Result<HttpRequest, HttpParseError>> {
pub fn read_file(file: &str) -> Vec<LineOutput> {
    let content = read_to_string(file).unwrap();
    content.split('\n').map(parse_line).collect()
}
