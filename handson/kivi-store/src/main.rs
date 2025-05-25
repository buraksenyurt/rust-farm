/*
    TCP protokolü ile çalışan hafifsiklet bir Key-Value Store klonu.
    Veri In-Memory olarak tutulur.
*/
mod data_store_tests;
mod handler;
mod server;
mod store;

fn main() -> std::io::Result<()> {
    server::run("127.0.0.1:5555")
}
