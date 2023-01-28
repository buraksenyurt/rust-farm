use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RegisteredUser {
    pub username: String,
    pub role: String,
}
