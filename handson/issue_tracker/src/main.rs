mod constants;
mod data;
mod issue;
mod json;
mod owner;
mod request_handler;
mod response;
mod test;
mod utility;

use crate::request_handler::{Handler, ReadRequestHandler, WriteResponseHandler};
use std::net::TcpListener;
use std::thread;

fn main() {
    let mut threads = Vec::new();

    let read_handler = ReadRequestHandler {
        host_address: "127.0.0.1:8086",
    };
    let write_handler = WriteResponseHandler {
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
            let stream = stream.unwrap();
            write_handler.handle(stream);
        }
    });
    threads.push(write_handle);

    for th in threads {
        th.join().unwrap();
    }
    println!("Program sonlanıyor");
}
