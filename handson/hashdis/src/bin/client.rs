use bytes::BytesMut;
use clap::Parser;
use hashdis::command::{Command, Keyword};
use log::{error, info, warn};
use std::io::Error;
use std::str::from_utf8;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
                    Ok(_) => {
                        let mut buffer = BytesMut::with_capacity(512);
                        s.read_buf(&mut buffer).await?;
                        if let Ok(r) = from_utf8(&buffer) {
                            println!("{}", r);
                        }
                        Ok(())
                    }
                    Err(e) => {
                        error!("Veri gönderilemedi. {}", e);
                        Err(e)
                    }
                },
                Command::Set { key, value } => {
                    s.write_all(b"set").await?;
                    s.write_all(b" ").await?;
                    s.write_all(key.as_bytes()).await?;
                    s.write_all(b" ").await?;
                    s.write_all(value.as_bytes()).await?;
                    Ok(())
                }
                Command::Get { key } => {
                    s.write_all(b"get").await?;
                    s.write_all(b" ").await?;
                    s.write_all(key.as_bytes()).await?;
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
            Err(e)
        }
    }
}
