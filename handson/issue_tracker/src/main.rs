mod constants;
mod data;
mod handlers;
mod issue;
mod json;
mod owner;
mod request;
mod response;
mod test;
mod utility;

use crate::data::IssueStore;
use crate::handlers::{Handler, ReadRequestHandler, WriteResponseHandler};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut threads = Vec::new();
    let mut store = IssueStore::default();
    store.seed();
    let data = Arc::new(Mutex::new(store.data));
    let data_1 = Arc::clone(&data);
    let data_2 = Arc::clone(&data);
    let mut read_handler = ReadRequestHandler {
        host_address: "127.0.0.1:8086",
    };
    let mut write_handler = WriteResponseHandler {
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
            let data = data_1.lock().unwrap();
            let stream = stream.unwrap();
            read_handler.handle(stream, data);
        }
    });
    threads.push(read_handle);

    let write_handle = thread::spawn(move || {
        for stream in writer.incoming() {
            let stream = stream.unwrap();
            let data = data_2.lock().unwrap();
            write_handler.handle(stream, data);
        }
    });
    threads.push(write_handle);

    for th in threads {
        th.join().unwrap();
    }
}
