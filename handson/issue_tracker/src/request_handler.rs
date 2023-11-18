use crate::constants::*;
use crate::data::*;
use crate::response::{HttpResponse, Response};
use crate::utility::write_std_response;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct RequestHandler<'a> {
    pub host_address: &'a str,
}

impl<'a> RequestHandler<'a> {
    pub fn handle(&self, mut stream: TcpStream) {
        let buffer_reader = BufReader::new(&mut stream);
        let request_line = buffer_reader.lines().next().unwrap().unwrap();
        println!("{}", request_line);
        if request_line == GET_HELLO {
            let response = Response::new(
                HttpResponse::Ok,
                "{\"response\":\"hello there\"}".to_string(),
            );
            stream.write_all(response.to_string().as_bytes()).unwrap();
        } else if request_line == GET_ALL_ISSUES {
            let dummy_issues = get_dummy_issues();
            let json_output = to_json_array(&dummy_issues);
            let response = Response::new(HttpResponse::Ok, json_output);
            stream.write_all(response.to_string().as_bytes()).unwrap();
        } else if request_line.starts_with(DELETE_ISSUE) {
            println!("Silme talebi geldi. {}", request_line);
            write_std_response(&mut stream, HttpResponse::Ok);
        } else if request_line.starts_with(GET_ISSUE) {
            println!("Tek issue talebi geldi {}", request_line);
            write_std_response(&mut stream, HttpResponse::Ok);
        } else {
            println!("Geçerli bir talep değil!");
            write_std_response(&mut stream, HttpResponse::NotFound);
        }
    }
}
