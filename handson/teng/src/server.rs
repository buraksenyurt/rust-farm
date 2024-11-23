use crate::file::*;
use axum::response::Html;
use crate::controller::invoice_controller::generate;

pub async fn generate_report() -> Html<String> {
    let invoice_data = load_json("data/invoice.json");
    let template_file = "templates/invoice.teng";
    let template = load_template(template_file);
    let output = generate(&template, &invoice_data);
    Html(output)
}
