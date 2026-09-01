use crate::command::Command;
use crate::store::DataStore;
use log::{error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[allow(dead_code)]
pub async fn handle_request(mut stream: TcpStream, data_store: DataStore) {
    let mut buffer = [0; 1024];
    loop {
        let size = match stream.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                error!("{}", e);
                break;
            }
        };

        info!("Read {}(bytes)", size);

        let request = String::from_utf8_lossy(&buffer[..size]);
        let cmd = Command::parse(&request);

        let response = match cmd {
            Command::Set { key, value } => {
                data_store.set(&key, &value).await;
                "OK\r\n".to_string()
            }
            Command::Get { key } => {
                info!("GET {}", key);
                match data_store.get(&key).await {
                    Some(value) => format!("{}\r\n", value),
                    None => "NOT FOUND\r\n".to_string(),
                }
            }
            Command::Remove { key } => {
                info!("REMOVE {}", key);
                if data_store.remove(&key).await {
                    "OK\r\n".to_string()
                } else {
                    "NOT FOUND\r\n".to_string()
                }
            }
            Command::List => {
                let keys = data_store.keys().await;
                format!("{}\r\n", keys.join("\r\n"))
            }
            Command::Invalid(cmd) => format!("ERROR: Unknown command '{}'\r\n", cmd),
        };

        if let Err(e) = stream.write_all(response.as_bytes()).await {
            error!("{}", e);
            break;
        }
    }
}
