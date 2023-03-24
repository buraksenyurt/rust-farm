use crate::bookshop::bookshop_server::BookshopServer;
use crate::bookshop::FILE_DESCRIPTOR_SET;
use crate::bookshop_service::BookShopService;
use tonic::transport::Server;

mod bookshop_service;

mod bookshop {
    include!("bookshop.rs");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("service_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_address = "127.0.0.1:4567".parse().unwrap();
    let bookshop = BookShopService::default();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    println!(
        "Belli kategori için rastgele kitap önerisi sunma hizmeti.\nBookShop gRPC servisi {} adresinden dinlemede...",
        service_address
    );
    Server::builder()
        .add_service(BookshopServer::new(bookshop))
        .add_service(reflection_service)
        .serve(service_address)
        .await?;

    Ok(())
}
