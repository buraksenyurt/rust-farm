use crate::handler::handle_request;
use crate::store::DataStore;
use std::net::TcpListener;
use std::thread;

pub fn run(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;
    let store = DataStore::new();

    println!("Server running at {}", address);

    for stream in listener.incoming() {
        let stream = stream?;
        let store = store.clone();

        thread::spawn(move || {
            handle_request(stream, store);
        });
    }

    Ok(())
}
