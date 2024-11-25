use crate::report_builder::*;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use crate::model::*;

mod file;
mod model;
mod report_builder;
mod traits;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/reports/invoice",
            get(|| async { generate_report::<Invoice>("invoice").await }),
        )
        .route(
            "/reports/sales/monthly",
            get(|| async {
                generate_report::<SalesData>("monthly_sales").await
            }),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Sunucu çalışıyor: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
