use crate::bookshop::bookshop_server::BookshopServer;
use crate::bookshop_service::BookShopService;
use tonic::transport::Server;

mod bookshop_service;

mod bookshop {
    include!("bookshop.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_address = "127.0.0.1:60001".parse().unwrap();
    let bookshop = BookShopService::default();
    println!(
        "Belli kategori için rastgele kitap önerisi sunma hizmeti.\nBookShop gRPC servisi {} adresinden dinlemede...",
        service_address
    );
    Server::builder()
        .add_service(BookshopServer::new(bookshop))
        .serve(service_address)
        .await?;

    Ok(())
}
