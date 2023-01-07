#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub(crate) url: String,
    pub(crate) method: Method,
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post { body: String },
}

// #[derive(Debug, PartialEq)]
// pub enum HttpParseError {
//     UnrecognizedMethod,
//     UnrecognizedFormat,
// }
