use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TopPlayers {
    pub players: Vec<Player>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub nickname: String,
    pub total_score: u32,
    pub join_year: u16,
}
