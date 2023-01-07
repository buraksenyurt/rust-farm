pub struct HttpRequest {
    pub(crate) url: String,
    pub(crate) method: Method,
}

pub enum Method {
    Get,
    Post { body: String },
}

pub enum HttpParseError {
    UnrecognizedMethod,
    UnrecognizedFormat,
}
