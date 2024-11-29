use crate::file::load_json;
use crate::model::SalesData;
use axum::response::Html;
use axum::routing::get;
use axum::{Extension, Router};
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
            eprintln!("Şablonları yüklerken hata oluştu: {}", e);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/reports/sales/monthly", get(generate_sales_report))
        .layer(Extension(tera));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Sunucu çalışıyor: http://{}", addr);
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
            eprintln!("Şablon render edilirken hata oluştu: {:?}", e);
            std::process::exit(1);
        })
        .unwrap();

    Html(rendered)
}