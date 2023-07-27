use crate::repository::GameRepository;
use crate::server::Server;
use crate::service::Service;
use almanac_data::get_conn;
use std::env;

mod dto;
mod entity;
mod messages;
mod repository;
mod server;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let conn_str = env::var("DATABASE_URL").expect("DATABASE_URL bilgisi eksik");
    let conn = get_conn(&conn_str).await?;
    let repository = GameRepository::new(conn.clone())
        .await
        .expect("Error creating repository");
    let service = Service::new(repository);
    let address=env::var("API_ADDRESS").expect("API_ADDRESS bilgisi eksik");
    let server=Server{
        address
    };
    server.run(service).await;
    Ok(())
}
