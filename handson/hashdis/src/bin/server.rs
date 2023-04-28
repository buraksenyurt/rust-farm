use hashdis::db::Db;
use hashdis::processor::{Processor, ServerProcessor};
use log::{error, info, warn};
use std::io::Error;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:6380").await;
    let processor = ServerProcessor;
    match listener {
        Ok(l) => {
            info!("hashdis dinlemede. 127.0.0.1:6380");
            let mut db = Db::default();
            loop {
                match l.accept().await {
                    Ok((mut socket, addr)) => {
                        warn!("{:?} {}", socket, addr);
                        processor.run(&mut socket, Some(&mut db)).await?;
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
