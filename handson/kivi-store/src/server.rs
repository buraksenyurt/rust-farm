use crate::handler::handle_request;
use crate::store::DataStore;
use log::info;
use tokio::net::TcpListener;

pub async fn run(address: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(address).await?;
    let store = DataStore::new();

    info!("Server running at {}", address);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {} connected", addr);
        let store = store.clone();
        tokio::spawn(async move {
            handle_request(stream, store).await;
        });
    }
}
