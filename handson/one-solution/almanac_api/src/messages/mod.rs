use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameRequest {
    pub title: String,
    pub point: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameResponse {
    pub id: i32,
    pub title: String,
    pub point: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGamePointRequest {
    pub id: i32,
    pub new_point: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGamePointResponse {
    pub id: i32,
    pub title: String,
    pub old_point: i32,
    pub new_point: i32,
}
