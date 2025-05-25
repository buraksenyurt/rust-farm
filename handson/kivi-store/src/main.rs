/*
    TCP protokolü ile çalışan hafifsiklet bir Key-Value Store klonu.
    Veri In-Memory olarak tutulur.
*/
mod command;
mod handler;
mod server;
mod store;
mod tests;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    env_logger::init();
    server::run("127.0.0.1:5555").await
}
