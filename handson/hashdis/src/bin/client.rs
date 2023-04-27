use clap::Parser;
use hashdis::command::{Command, Keyword};
use log::{error, info, warn};
use std::io::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    env_logger::init();
    let tcp_stream = TcpStream::connect("127.0.0.1:6380").await;
    match tcp_stream {
        Ok(mut s) => {
            info!("Hashdis bağlantısı sağlandı.");
            let arguments = Keyword::parse();
            match arguments.command {
                Command::Ping => match s.write_all(b"ping").await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        error!("Veri gönderilemedi. {}", e);
                        return Err(e);
                    }
                },
                Command::Set { key, value } => {
                    s.write_all(b"set").await?;
                    s.write_all(b" ").await?;
                    s.write_all(&key.as_bytes()).await?;
                    s.write_all(b" ").await?;
                    s.write_all(&value.as_bytes()).await?;
                    Ok(())
                }
                Command::Get { key } => {
                    s.write_all(b"get").await?;
                    s.write_all(b" ").await?;
                    s.write_all(&key.as_bytes()).await?;
                    Ok(())
                }
                _ => {
                    warn!("Komut anlaşılamadı");
                    Ok(())
                }
            }
        }
        Err(e) => {
            error!("Sunucu ile bağlantı hatası. {}", e);
            return Err(e);
        }
    }
}
