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
