use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct SignInResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct IdentityResponse {
    pub id: i32,
    pub email: String,
    pub firstname: String,
    pub surname: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeveloperListResponse {
    pub total_count: usize,
    pub developers: Vec<DeveloperResponse>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeveloperResponse {
    pub id: i32,
    pub fullname: String,
    pub about: String,
    pub level: i16,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GameListResponse {
    pub total_count: usize,
    pub games: Vec<GameResponse>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GameResponse {
    pub id: i32,
    pub developer_id: i32,
    pub title: String,
    pub summary: String,
    pub year: String,
}
