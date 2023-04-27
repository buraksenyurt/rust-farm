use log::{error, info};
use std::io;
use std::io::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    env_logger::init();
    let stdin = io::stdin();
    let tcp_stream = TcpStream::connect("127.0.0.1:6380").await;
    match tcp_stream {
        Ok(mut s) => {
            info!("Hashdis bağlantısı sağlandı. Bir şeyler yazıp Enter'a basın.");
            let mut input = String::new();
            let read_line = stdin.read_line(&mut input);
            match read_line {
                Ok(_) => match s.write_all(input.as_bytes()).await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        error!("Veri gönderilemedi. {}", e);
                        return Err(e);
                    }
                },
                Err(e) => {
                    error!("Veri gönderilemedi. {}", e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            error!("Sunucu ile bağlantı hatası. {}", e);
            return Err(e);
        }
    }
}
