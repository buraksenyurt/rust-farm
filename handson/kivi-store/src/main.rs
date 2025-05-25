/*
    TCP protokolü ile çalışan hafifsiklet bir Key-Value Store klonu.
    Veri In-Memory olarak tutulur.
*/
use std::env;

mod command;
mod handler;
mod server;
mod store;
mod tests;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let address = env::var("LISTEN_ADDRESS").unwrap_or_else(|_| "127.0.0.1:5555".to_string());
    server::run(&address).await
}
