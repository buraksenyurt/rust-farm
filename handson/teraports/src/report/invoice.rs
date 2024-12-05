use crate::ds::*;
use crate::model::Invoice;
use crate::report::generate_report;
use axum::response::Html;
use std::sync::Arc;
use tera::Tera;

pub async fn generate_invoice_report(tera: Arc<Tera>) -> Html<String> {
    let jds = JsonDataSource {
        file_name: "data/invoice.json".to_string(),
    };
    let invoice_data: Invoice = jds.load_data().await;
    generate_report(tera, "invoice.html", invoice_data).await
}
