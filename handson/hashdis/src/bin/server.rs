use bytes::BytesMut;
use hashdis::command::{ServerCommand};
use hashdis::utility::Utility;
use log::{error, info, warn};
use std::io::Error;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:6380").await;
    match listener {
        Ok(l) => {
            info!("hashdis dinlemede. 127.0.0.1:6380");

            loop {

                match l.accept().await {
                    Ok((mut socket, addr)) => {
                        warn!("{:?} {}", socket, addr);

                        let mut buffer = BytesMut::with_capacity(512);
                        socket.read_buf(&mut buffer).await?;
                        let incoming_commands = Utility::convert_to_vec(&mut buffer);
                        let first_command = ServerCommand::from_str(incoming_commands[0].as_str());

                        if let Ok(c) = first_command {
                            info!("İstemciden gelen içerik {:?}", incoming_commands);
                            match c {
                                ServerCommand::Ping => {
                                    println!("İstemci ping attı...");
                                    socket.write_all(b"pong").await?;
                                }
                                ServerCommand::Get => {}
                                ServerCommand::Set => {}
                                ServerCommand::Unknown => {}
                            }
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
            Err(e)
        }
    }
}
