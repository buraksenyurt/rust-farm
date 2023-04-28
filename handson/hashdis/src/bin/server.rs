use hashdis::db::Db;
use hashdis::processor::{Processor, ServerProcessor};
use log::{error, info, warn};
use std::io::Error;
use tokio::net::TcpListener;
use tokio::signal;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    env_logger::init();
    let mut listener = TcpListener::bind("127.0.0.1:6380")
        .await
        .expect("listener oluşturulamadı");
    let shutdown = signal::ctrl_c();
    tokio::select! {
        result = run(&mut listener) => {
            if let Err(e)=result{
                error!("{}",e);
                println!("{}",e);
            }
        }
        _=shutdown => {
            warn!("kapatılıyor");
            println!(" kapatılıyor...");
        }
    }
    Ok(())
}

pub async fn run(listener: &mut TcpListener) -> std::io::Result<()> {
    let mut db = Db::default();
    let processor = ServerProcessor;
    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                info!("{:?} {}", socket, addr);
                processor.run(&mut socket, Some(&mut db)).await?;
                continue;
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    }
}
