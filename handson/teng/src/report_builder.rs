use crate::file::*;
use crate::model::invoice::Invoice;
use crate::model::sales::SalesData;
use axum::response::Html;

pub async fn generate_invoice_report() -> Html<String> {
    let invoice_data: Invoice = load_json("data/invoice.json");
    let template_file = "templates/invoice.teng";
    let template = load_template(template_file);
    let output = crate::controller::invoice_controller::generate(&template, &invoice_data);
    Html(output)
}

pub async fn generate_sales_report() -> Html<String> {
    let sales_data: SalesData = load_json("data/monthly_sales.json");
    let template_file = "templates/monthly_sales.teng";
    let template = load_template(template_file);
    let output = crate::controller::sales_controller::generate(&template, &sales_data);
    Html(output)
}
