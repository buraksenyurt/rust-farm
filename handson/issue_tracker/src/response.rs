use crate::constants::*;
use std::fmt::{Display, Formatter};

pub struct Response {
    pub http_response: HttpResponse,
    pub content: String,
}

impl Response {
    pub fn new(http_response: HttpResponse, content: String) -> Self {
        Self {
            http_response,
            content,
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}\r\n\r\n{}",
            self.http_response, CONTENT_TYPE, self.content
        )
    }
}

pub enum HttpResponse {
    Ok,
    NotFound,
    Created,
    BadRequest,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            HttpResponse::Ok => HTTP_OK,
            HttpResponse::Created => HTTP_CREATED,
            HttpResponse::NotFound => HTTP_NOT_FOUND,
            HttpResponse::BadRequest => HTTP_BAD_REQUEST,
        };
        write!(f, "{}\r\n", output)
    }
}
