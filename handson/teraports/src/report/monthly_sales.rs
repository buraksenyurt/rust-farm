use crate::ds::*;
use crate::model::SalesData;
use crate::report::generate_report;
use axum::response::Html;
use std::sync::Arc;
use tera::Tera;

pub async fn generate_sales_report(tera: Arc<Tera>) -> Html<String> {
    let jds = JsonDataSource {
        file_name: "data/monthly_sales.json".to_string(),
    };
    let sales_data: SalesData = jds.load_data().await;
    generate_report(tera, "monthly_sales.html", sales_data).await
}
