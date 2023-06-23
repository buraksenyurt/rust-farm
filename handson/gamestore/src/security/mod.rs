use crate::app_settings::AppSettings;
use crate::jwt::claims::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    serde::{Deserialize, Serialize},
};

pub struct AuthenticatedUser {
    pub user_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(token) = req.headers().get_one("token") {
            let settings = req.rocket().state::<AppSettings>().unwrap();
            let data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(settings.jwt_secret.as_bytes()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            );
            let claims = match data {
                Ok(p) => p.claims,
                Err(_) => {
                    return Outcome::Failure((Status::Unauthorized, "Geçersiz token".to_string()))
                }
            };
            Outcome::Success(AuthenticatedUser {
                user_id: claims.sub,
            })
        } else {
            Outcome::Failure((Status::Unauthorized, "Token bilgisi eksik".to_string()))
        }
    }
}
