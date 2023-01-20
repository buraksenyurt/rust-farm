use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct UserDao {
    pub username: String,
    pub password: String,
    pub role: String,
}
