use crate::command::Command;
use crate::store::DataStore;
use std::io::{Read, Write};
use std::net::TcpStream;

#[allow(dead_code)]
pub fn handle_request(mut stream: TcpStream, data_store: DataStore) {
    let mut buffer = [0; 1024];
    while let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            break;
        }

        let request = String::from_utf8_lossy(&buffer[..size]);
        let cmd = Command::parse(&request);

        let response = match cmd {
            Command::Set { key, value } => {
                data_store.set(&key, &value);
                "OK\n".to_string()
            }
            Command::Get { key } => data_store
                .get(&key)
                .unwrap_or_else(|| "NOT FOUND\n".to_string()),
            Command::Remove { key } => {
                if data_store.remove(&key) {
                    "OK\n".to_string()
                } else {
                    "NOT FOUND\n".to_string()
                }
            }
            Command::List => data_store.keys().join("\n").to_string(),
            Command::Invalid(cmd) => format!("ERROR: Unknown command '{}'\n", cmd),
        };

        let _ = stream.write_all(response.as_bytes());
    }
}
