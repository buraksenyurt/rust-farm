use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
    pub role: Option<String>,
}
