use crate::report::*;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tera::Tera;

mod ds;
mod model;
mod report;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => Arc::new(t),
        Err(e) => {
            eprintln!("Error on loading templates: {}", e);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/", {
            let tera = Arc::clone(&tera);
            get(move || async move { home(tera).await })
        })
        .route("/reports/sales/monthly", {
            let tera = Arc::clone(&tera);
            get(move || async move { generate_sales_report(tera).await })
        })
        .route("/reports/invoice", {
            let tera = Arc::clone(&tera);
            get(move || async move { generate_invoice_report(tera).await })
        })
        .route("/reports/games/top", {
            let tera = Arc::clone(&tera);
            get(move || async move { generate_top_games_report(tera).await })
        })
        .route("/reports/players/top", {
            let tera = Arc::clone(&tera);
            get(move || async move { generate_top_players_report(tera).await })
        });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server online http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
