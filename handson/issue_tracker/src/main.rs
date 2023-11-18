mod constants;
mod data;
mod issue;
mod json;
mod owner;
mod request_handler;
mod response;
mod test;
mod utility;

use crate::constants::*;
use crate::issue::Issue;
use crate::json::Deserializer;
use crate::request_handler::RequestHandler;
use crate::response::HttpResponse;
use crate::utility::write_std_response;
use std::io::Read;
use std::net::TcpListener;
use std::thread;

fn main() {
    let mut threads = Vec::new();

    let read_handler = RequestHandler {
        host_address: "127.0.0.1:8086",
    };
    let write_handler = RequestHandler {
        host_address: "127.0.0.1:8087",
    };
    let listener = TcpListener::bind(read_handler.host_address).unwrap();
    println!(
        "Okuma yapan sunucu {} adresinden ayakta.",
        read_handler.host_address
    );
    let writer = TcpListener::bind(write_handler.host_address).unwrap();
    println!(
        "Yazma yapan sunucu {} adresinden ayakta.",
        write_handler.host_address
    );

    let read_handle = thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            read_handler.handle(stream);
        }
    });
    threads.push(read_handle);

    let write_handle = thread::spawn(move || {
        for stream in writer.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024];
            let data_size = stream.read(&mut buffer).unwrap();
            println!("{} byte içerik geldi", data_size);
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
                //println!("{}", json_body);
                let issue = <Issue as Deserializer>::from(json_body.as_str()).unwrap();
                println!("Deserialized:\n{:?}", issue);
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
