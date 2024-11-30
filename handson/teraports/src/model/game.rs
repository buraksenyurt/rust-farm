use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TopGames {
    games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub developer: String,
    pub genre: String,
    pub rating: f64,
}
