use bytes::BytesMut;
use log::{error, info, warn};
use std::io::Error;
use tokio::io::AsyncReadExt;
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
                        warn!("İstemciden gelen mesaj {:?}", buffer);
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
