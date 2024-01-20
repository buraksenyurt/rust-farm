use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_websockets::{Error, ServerBuilder};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let address = "127.0.0.1:9001";
    let listener = TcpListener::bind(&address).await?;
    println!("WebSocket {address} adresinden dinlemede");

    // İstekleri dinlemek için sonsuz bir döngü
    loop {
        let (conn, _) = listener.accept().await?;
        let mut server = ServerBuilder::new().accept(conn).await?;

        while let Some(Ok(item)) = server.next().await {
            println!("Gelen bilgi: {item:?}");
            // Gelen bilgi gönderen istemciye geri döndürülür. Bir nevi Echo server
            server.send(item).await?;
        }
    }
}
