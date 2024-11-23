use crate::server::*;
use axum::{extract::Query, response::Html, routing::get, Router};
use std::net::SocketAddr;

mod file;
mod model;
mod report;
mod server;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/report", get(generate_report));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Sunucu çalışıyor: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
