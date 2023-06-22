use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub surname: String,
}
