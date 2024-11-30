use crate::ds::{load_data, JsonSource};
use crate::model::TopGames;
use crate::report::generate_report;
use axum::response::Html;
use std::sync::Arc;
use tera::Tera;

pub async fn generate_top_games_report(tera: Arc<Tera>) -> Html<String> {
    let json_source = JsonSource {
        file_name: "data/top_games.json".to_string(),
    };
    let invoice_data: TopGames = load_data(json_source).await;
    generate_report(tera, "top_games.html", invoice_data).await
}
