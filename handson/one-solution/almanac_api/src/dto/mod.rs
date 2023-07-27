use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: i32,
    pub title: String,
    pub point: i32,
}
