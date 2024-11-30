use crate::ds::*;
use crate::model::SalesData;
use crate::report::generate_report;
use axum::response::Html;
use std::sync::Arc;
use tera::Tera;

pub async fn generate_sales_report(tera: Arc<Tera>) -> Html<String> {
    let json_source = JsonSource {
        file_name: "data/monthly_sales.json".to_string(),
    };
    let sales_data: SalesData = load_data(json_source).await;
    generate_report(tera, "monthly_sales.html", sales_data).await
}
