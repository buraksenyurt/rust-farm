use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
}
