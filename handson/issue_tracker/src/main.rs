use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let conn = Connection {
        host_address: "127.0.0.1:8086",
    };
    let listener = TcpListener::bind(conn.host_address).unwrap();
    println!("Sunucu dinlemede {}", conn.host_address);
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
        let http_request: Vec<_> = buffer_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Talep -> {:#?}", http_request);

        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        let content = "{'response':'hello there'}";
        let length = content.len();
        let response=format!("{status_line}\r\nContent-Type: application/json\r\nContent-Length: {length}\r\n{content}\r\n");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
