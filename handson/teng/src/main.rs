use crate::report_builder::*;
use axum::{routing::get, Router};
use std::net::SocketAddr;

mod controller;
mod file;
mod model;
mod report_builder;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/reports/invoice", get(generate_invoice_report))
        .route("/reports/sales/monthly", get(generate_sales_report));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Sunucu çalışıyor: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
