/*
    TCP protokolü ile çalışan hafifsiklet bir Key-Value Store klonu.
    Veri In-Memory olarak tutulur.
*/
mod command;
mod handler;
mod server;
mod store;
mod tests;

fn main() -> std::io::Result<()> {
    server::run("127.0.0.1:5555")
}
