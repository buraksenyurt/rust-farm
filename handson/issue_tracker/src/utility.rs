use crate::response::{HttpResponse, Response};
use std::io::Write;
use std::net::TcpStream;

pub fn write_std_response(stream: &mut TcpStream, res: HttpResponse) {
    let response = Response::new(res, String::default());
    stream.write_all(response.to_string().as_bytes()).unwrap();
}
