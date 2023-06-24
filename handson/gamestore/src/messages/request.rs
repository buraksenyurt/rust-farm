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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateDeveloperRequest {
    pub fullname: String,
    pub about: String,
    pub level: i16,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateGameRequest {
    pub developer_id: i32,
    pub title: String,
    pub year: String,
    pub summary: String,
}
