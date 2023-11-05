use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
const HTTP_OK: &str = "HTTP/1.1 200 OK";
const HTTP_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";
const CONTENT_TYPE: &str = "Content-Type: application/json";

fn main() {
    let conn = Connection {
        host_address: "127.0.0.1:8086",
    };
    let listener = TcpListener::bind(conn.host_address).unwrap();
    println!("Sunucu dinliyor: {}", conn.host_address);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        conn.handle(stream);
    }
}

struct Connection<'a> {
    pub host_address: &'a str,
}

impl<'a> Connection<'a> {
    fn handle(&self, mut stream: TcpStream) {
        let buffer_reader = BufReader::new(&mut stream);
        let request_line = buffer_reader.lines().next().unwrap().unwrap();
        println!("Talep -> {:#?}", request_line);
        if request_line == "GET /hello HTTP/1.1" {
            let response = Response::new(
                HttpResponse::Ok,
                "{\"response\":\"hello there\"}".to_string(),
            );
            stream.write_all(response.to_string().as_bytes()).unwrap();
        } else if request_line == "GET /issues HTTP/1.1" {
            let response =
                Response::new(HttpResponse::Ok, "{\"issues\":\"issues list\"}".to_string());
            stream.write_all(response.to_string().as_bytes()).unwrap();
        } else {
            println!("Geçerli bir talep değil!");
            let response = Response::new(HttpResponse::NotFound, String::default());
            stream.write_all(response.to_string().as_bytes()).unwrap();
        }
    }
}

struct Response {
    http_response: HttpResponse,
    content: String,
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

enum HttpResponse {
    Ok,
    NotFound,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            HttpResponse::Ok => HTTP_OK,
            HttpResponse::NotFound => HTTP_NOT_FOUND,
        };
        write!(f, "{}\r\n", output)
    }
}
