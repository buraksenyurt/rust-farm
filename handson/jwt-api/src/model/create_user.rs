use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub role: String,
}
