use crate::file::*;
use crate::traits::Reportable;
use axum::response::Html;

pub async fn generate_report<T>(report_name: &str) -> Html<String>
where
    T: Reportable + serde::Serialize + serde::de::DeserializeOwned,
{
    let data: T = load_json(format!("data/{report_name}.json").as_str());
    let template = load_template(format!("templates/{report_name}.teng").as_str());
    let output = T::generate(&template, &data);
    Html(output)
}
