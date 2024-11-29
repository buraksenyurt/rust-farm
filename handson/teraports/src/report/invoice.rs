use crate::file::load_json;
use crate::model::Invoice;
use crate::report::generate_report;
use axum::response::Html;
use std::sync::Arc;
use tera::Tera;

pub async fn generate_invoice_report(tera: Arc<Tera>) -> Html<String> {
    let invoice_data = load_json::<Invoice>("data/invoice.json");
    generate_report(tera, "invoice.html", invoice_data).await
}
