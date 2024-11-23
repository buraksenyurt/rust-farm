use crate::file::*;
use crate::report::*;
use axum::response::Html;

pub async fn generate_report() -> Html<String> {
    let invoice_data = load_json("data_samples/invoice.json");
    let template_file = "templates/invoice.teng";
    let template = load_template(template_file);
    let output = generate(&template, &invoice_data);
    Html(output)
}
