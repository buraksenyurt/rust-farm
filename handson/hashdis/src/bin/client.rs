use hashdis::processor::{ClientProcessor, Processor};
use log::{error, info};
use std::io::Error;
use tokio::net::TcpStream;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    env_logger::init();

    let tcp_stream = TcpStream::connect("127.0.0.1:6380").await;
    let processor = ClientProcessor;
    match tcp_stream {
        Ok(mut s) => {
            info!("Hashdis bağlantısı sağlandı.");
            processor.run(&mut s, None).await?;
            Ok(())
        }
        Err(e) => {
            error!("Sunucu ile bağlantı hatası. {}", e);
            Err(e)
        }
    }
}
