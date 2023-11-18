use crate::issue::Issue;
use crate::json::Serializer;
use crate::response::{HttpResponse, Response};
use std::io::Write;
use std::net::TcpStream;

pub struct Utility {}

impl Utility {
    pub fn send_response(stream: &mut TcpStream, content: String, res: HttpResponse) {
        let response = Response::new(res, content);
        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
    pub fn to_json(issues: Vec<Issue>) -> String {
        let mut json_array = String::from("[\n");
        for (i, issue) in issues.iter().enumerate() {
            json_array.push_str(&issue.to_json());
            if i < issues.len() - 1 {
                json_array.push_str(",\n");
            }
        }
        json_array.push(']');
        json_array
    }
}
