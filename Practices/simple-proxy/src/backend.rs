use crate::request::Request;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::str::FromStr;

mod backend_test;
mod request;

fn main() {
    let port = 3002;
    let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let listener = TcpListener::bind(socket_address).unwrap();
    println!(
        "Origin/Backend Sunucu localhost:{} adresinden dinlemede...",
        port
    );

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request_message =
            if let Some(line) = std::str::from_utf8(&buffer).unwrap().lines().next() {
                line
            } else {
                println!("Geçersiz talep.");
                ""
            };
        let request_header = Request::from_str(request_message).unwrap();
        println!("Request Message {}", request_message);
        let response;
        if request_header.method.unwrap() != "GET".to_string()
            || !request_header
                .path
                .unwrap()
                .starts_with("/salary/categories/")
        {
            let message = "Invalid route";
            response = format!(
                "HTTP/1.1 400 Bad Request\nContent Type:text/html\nContent-Length:{}\n\n{}",
                message.len(),
                message
            );
        } else {
            let message = format!("Total Salary of category is 1000 Unit");
            response = format!(
                "HTTP/1.1 200 OK\nContent-Type:text/html\nContent-Length:{}\n\n{}",
                message.len(),
                message
            );
        }
        stream.write(response.as_bytes()).unwrap();
    }
}
