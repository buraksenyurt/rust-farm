use crate::transport::http::*;

pub fn parse_line(line: &str) -> Result<HttpRequest, HttpParseError> {
    let vec: Vec<&str> = line.split(' ').collect();
    match vec.as_slice() {
        ["GET", url] => Ok(HttpRequest {
            method: Method::Get,
            url: url.to_string(),
        }),
        ["POST", url, body @ ..] => Ok(HttpRequest {
            method: Method::Post {
                body: body.join(" "),
            },
            url: url.to_string(),
        }),
        [_header, ..] => Err(HttpParseError::UnrecognizedFormat),
        _ => Err(HttpParseError::UnrecognizedMethod),
    }
}
