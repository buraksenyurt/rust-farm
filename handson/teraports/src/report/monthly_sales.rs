use crate::file::load_json;
use crate::model::SalesData;
use crate::report::generate_report;
use axum::response::Html;
use std::sync::Arc;
use tera::Tera;

pub async fn generate_sales_report(tera: Arc<Tera>) -> Html<String> {
    let sales_data = load_json::<SalesData>("data/monthly_sales.json");
    generate_report(tera, "monthly_sales.html", sales_data).await
}
