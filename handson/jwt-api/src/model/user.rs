use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct User {
    pub id: usize,
    pub username: String,
    pub password: String,
    pub role: String,
}
