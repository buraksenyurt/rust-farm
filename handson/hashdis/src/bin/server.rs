use bytes::BytesMut;
use hashdis::command::Command;
use hashdis::utility::Utility;
use log::{error, info, warn};
use std::io::Error;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    env_logger::init();

    // localhost:6380 portunu dinlemek üzere tcp dinleyici nesnesi oluşturulur
    let listener = TcpListener::bind("127.0.0.1:6380").await;
    match listener {
        Ok(l) => {
            info!("hashdis dinlemede. 127.0.0.1:6380");
            loop {
                match l.accept().await {
                    Ok((mut socket, addr)) => {
                        info!("{:?} {}", socket, addr);
                        let mut buffer = BytesMut::with_capacity(512);
                        socket.read_buf(&mut buffer).await?;
                        let keywords = Utility::convert_to_vec(&mut buffer);
                        let command = Command::from_str(keywords[0].as_str());
                        match command {
                            Ok(c) => {
                                warn!("İstemciden gelen mesaj {:#?}", keywords);
                                match c {
                                    Command::Ping => {
                                        println!("{:?}", keywords);
                                        socket.write_all(b"pong").await?;
                                    }
                                    Command::Get { .. } => {}
                                    Command::Set { .. } => {}
                                    Command::Unknown => {}
                                }
                            }
                            Err(_) => {}
                        }
                        continue;
                    }
                    Err(e) => {
                        error!("{}", e);
                        return Err(e);
                    }
                }
            }
        }
        Err(e) => {
            error!("Sunucu hatası. {:?}", e);
            return Err(e);
        }
    };
}
