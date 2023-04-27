use log::{error, info};
use std::io::Error;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    env_logger::init();

    // localhost:6380 portunu dinlemek üzere tcp dinleyici nesnesi oluşturulur
    let listener = TcpListener::bind("127.0.0.1:6380").await;
    match listener {
        Ok(l) => {
            info!("hashdis dinlemede. 127.0.0.1:6380");
            // Program kapatılmadığı sürece sürekli dinelemede kalınması için sonsuz döngü
            match l.accept().await {
                Ok((socket, addr)) => {
                    info!("{:?} {}", socket, addr);
                    return Ok(());
                }
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            };
        }
        Err(e) => {
            error!("Sunucu hatası. {:?}", e);
            return Err(e);
        }
    }
}
