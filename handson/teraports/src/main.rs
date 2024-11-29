use axum::response::Html;
use axum::routing::get;
use axum::{Extension, Router};
use file::load_json;
use model::*;
use std::net::SocketAddr;
use std::sync::Arc;
use tera::{Context, Tera};

mod file;
mod model;

#[tokio::main]
async fn main() {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => Arc::new(t),
        Err(e) => {
            eprintln!("Error on loading templates {}", e);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/reports/sales/monthly", get(generate_sales_report))
        .route("/reports/invoice", get(generate_invoice_report))
        .layer(Extension(tera));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server online http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn generate_sales_report(Extension(tera): Extension<Arc<Tera>>) -> Html<String> {
    let sales_data = load_json::<SalesData>("data/monthly_sales.json");

    let mut context = Context::new();
    context.insert("report_title", &sales_data.report_title);
    context.insert("month", &sales_data.month);
    context.insert("total_sales", &sales_data.total_sales);
    context.insert("sales_by_category", &sales_data.sales_by_category);

    let rendered = tera
        .render("monthly_sales.html", &context)
        .map_err(|e| {
            eprintln!("Error on render operation: {:?}", e);
            std::process::exit(1);
        })
        .unwrap();

    Html(rendered)
}

async fn generate_invoice_report(Extension(tera): Extension<Arc<Tera>>) -> Html<String> {
    let sales_data = load_json::<Invoice>("data/invoice.json");

    let mut context = Context::new();
    context.insert("title", &sales_data.title);
    context.insert("customer", &sales_data.customer);
    context.insert("total_amount", &sales_data.total_amount);
    context.insert("serial_number", &sales_data.serial_number);
    context.insert("line_items", &sales_data.line_items);

    let rendered = tera
        .render("invoice.html", &context)
        .map_err(|e| {
            eprintln!("Error on render operation: {:?}", e);
            std::process::exit(1);
        })
        .unwrap();

    Html(rendered)
}
