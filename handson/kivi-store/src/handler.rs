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
        let mut parts = request.trim().splitn(3, ' ');

        let cmd = parts.next().unwrap_or("").to_uppercase();
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");

        let response = match cmd.as_str() {
            "SET" => {
                data_store.set(&key, &value);
                "OK\n".to_string()
            }
            "GET" => data_store
                .get(&key)
                .unwrap_or_else(|| "NOT FOUND\n".to_string()),
            "REMOVE" => {
                if data_store.remove(key) {
                    "OK\n".to_string()
                } else {
                    "NOT FOUND\n".to_string()
                }
            }
            "LIST" => data_store.keys().join("\n").to_string(),
            _ => "ERROR: Unknown command\n".to_string(),
        };

        let _ = stream.write_all(response.as_bytes());
    }
}
