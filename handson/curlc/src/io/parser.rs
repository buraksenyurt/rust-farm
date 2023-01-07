use crate::io::parser::LineOutput::Comment;
use crate::transport::http::*;

//pub fn parse_line(line: &str) -> Result<HttpRequest, HttpParseError> {
pub fn parse_line(line: &str) -> LineOutput {
    let vec: Vec<&str> = line.split(' ').collect();
    match vec.as_slice() {
        ["GET", url] => LineOutput::Valid(HttpRequest {
            method: Method::Get,
            url: url.to_string(),
        }),
        ["POST", url, body @ ..] => LineOutput::Valid(HttpRequest {
            method: Method::Post {
                body: body.join(" "),
            },
            url: url.to_string(),
        }),
        ["#", ..] => Comment,
        _=> LineOutput::None
        // [_header, ..] => Err(HttpParseError::UnrecognizedFormat),
        // _ => Err(HttpParseError::UnrecognizedMethod),
    }
}

#[derive(Debug, PartialEq)]
pub enum LineOutput {
    Valid(HttpRequest),
    Comment,
    None,
}
