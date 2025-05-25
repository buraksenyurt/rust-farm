use crate::handler::handle_request;
use crate::store::DataStore;
use log::{error, info};
use std::net::TcpListener;
use std::thread;

pub fn run(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;
    let store = DataStore::new();

    info!("Server running at {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = store.clone();
                thread::spawn(move || {
                    info!(
                        "Connection from {}",
                        stream
                            .peer_addr()
                            .unwrap_or_else(|_| "[Unknown]".parse().unwrap())
                    );
                    handle_request(stream, store);
                });
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }

    Ok(())
}
