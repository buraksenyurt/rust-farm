use crate::ds::{JsonDataSource, JsonLoader};
use crate::model::TopGames;
use crate::report::generate_report;
use axum::response::Html;
use std::sync::Arc;
use tera::Tera;

pub async fn generate_top_games_report(tera: Arc<Tera>) -> Html<String> {
    let jds = JsonDataSource {
        file_name: "data/top_games.json".to_string(),
    };
    let top_games: TopGames = jds.load_data().await;
    generate_report(tera, "top_games.html", top_games).await
}
