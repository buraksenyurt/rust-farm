use crate::report::*;
use axum::routing::get;
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tera::Tera;

mod ds;
mod model;
mod report;

#[tokio::main]
async fn main() {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => Arc::new(t),
        Err(e) => {
            eprintln!("Error on loading templates: {}", e);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/", {
            let tera = tera.clone();
            get(move || {
                let tera = Arc::clone(&tera);
                async move { report::home(tera).await }
            })
        })
        .route("/reports/sales/monthly", {
            let tera = tera.clone();
            get(move || {
                let tera = Arc::clone(&tera);
                async move { generate_sales_report(tera).await }
            })
        })
        .route("/reports/invoice", {
            let tera = tera.clone();
            get(move || {
                let tera = Arc::clone(&tera);
                async move { generate_invoice_report(tera).await }
            })
        })
        .layer(Extension(tera));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server online http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
