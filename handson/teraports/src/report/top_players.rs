use crate::ds::*;
use crate::model::*;
use crate::report::generate_report;
use axum::response::Html;
use std::sync::Arc;
use tera::Tera;

pub async fn generate_top_players_report(tera: Arc<Tera>) -> Html<String> {
    let csv = CsvDataSource {
        file_name: "data/top_players.txt".to_string(),
    };
    let players: Vec<Player> = csv.load_data().await;
    let top_players = TopPlayers { players };
    generate_report(tera, "top_players.html", top_players).await
}
