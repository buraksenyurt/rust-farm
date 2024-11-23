use crate::server::*;
use axum::{routing::get, Router};
use std::net::SocketAddr;

mod file;
mod server;
mod model;
mod controller;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/reports/invoice", get(generate_report));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Sunucu çalışıyor: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
