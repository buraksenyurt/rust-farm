mod constants;
mod data;
mod issue;
mod json;
mod owner;
mod response;
mod test;

use crate::constants::*;
use crate::data::*;
use crate::response::{HttpResponse, Response};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let mut threads = Vec::new();

    let conn_read = Connection {
        host_address: "127.0.0.1:8086",
    };
    let conn_write = Connection {
        host_address: "127.0.0.1:8087",
    };
    let listener = TcpListener::bind(conn_read.host_address).unwrap();
    println!(
        "Okuma yapan sunucu {} adresinden ayakta.",
        conn_read.host_address
    );
    let writer = TcpListener::bind(conn_write.host_address).unwrap();
    println!(
        "Yazma yapan sunucu {} adresinden ayakta.",
        conn_write.host_address
    );

    let read_handle = thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            conn_read.handle(stream);
        }
    });
    threads.push(read_handle);

    let write_handle = thread::spawn(move || {
        for stream in writer.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let input = String::from_utf8_lossy(&buffer[..]).to_string();
            let line_count = input.lines().count();
            println!("{}\n", input);
            if input.starts_with(POST_ISSUE) {
                let mut json_body = String::new();
                input
                    .lines()
                    .skip(POST_SKIP_COUNT)
                    .take(line_count - POST_SKIP_COUNT)
                    .for_each(|line| json_body.push_str(line.trim()));
                println!("{}", json_body);

                write_std_response(&mut stream, HttpResponse::Created);
            }
        }
    });
    threads.push(write_handle);

    for th in threads {
        th.join().unwrap();
    }
    println!("Program sonlanıyor");
}

struct Connection<'a> {
    pub host_address: &'a str,
}

impl<'a> Connection<'a> {
    fn handle(&self, mut stream: TcpStream) {
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

fn write_std_response(stream: &mut TcpStream, res: HttpResponse) {
    let response = Response::new(res, String::default());
    stream.write_all(response.to_string().as_bytes()).unwrap();
}
