use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct SignInResponse {
    pub token: String,
}
