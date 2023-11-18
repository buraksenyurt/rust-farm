use crate::response::{HttpResponse, Response};
use std::io::Write;
use std::net::TcpStream;
pub struct Utility {}

impl Utility {
    pub fn send_response(stream: &mut TcpStream, content: String, res: HttpResponse) {
        let response = Response::new(res, content);
        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}
