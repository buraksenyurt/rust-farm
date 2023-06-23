use crate::app_settings::AppSettings;
use crate::controller::{ErrorResponse, Response, SuccessResponse};
use crate::entity::prelude::User;
use crate::entity::user;
use crate::jwt::claims::Claims;
use crate::messages::{IdentityResponse, SignInRequest, SignInResponse, SignUpRequest};
use crate::security::AuthenticatedUser;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::*;
use std::time::SystemTime;

#[get("/identity")]
pub async fn identity(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Response<Json<IdentityResponse>> {
    let db = db as &DatabaseConnection;
    let usr = User::find_by_id(user.user_id).one(db).await?.unwrap();

    Ok(SuccessResponse((
        Status::Ok,
        Json(IdentityResponse {
            id: usr.id,
            email: usr.email,
            firstname: usr.first_name,
            surname: usr.sur_name,
        }),
    )))
}

#[post("/sign-in", data = "<sign_in_request>")]
pub async fn sign_in(
    db: &State<DatabaseConnection>,
    settings: &State<AppSettings>,
    sign_in_request: Json<SignInRequest>,
) -> Response<Json<SignInResponse>> {
    let db = db as &DatabaseConnection;
    let settings = settings as &AppSettings;
    let usr = match User::find()
        .filter(user::Column::Email.eq(&sign_in_request.email))
        .one(db)
        .await?
    {
        Some(usr) => usr,
        None => {
            return Err(ErrorResponse((
                Status::Unauthorized,
                "Geçersiz ehliyet".to_string(),
            )))
        }
    };
    if !verify(&sign_in_request.password, &usr.password).unwrap() {
        return Err(ErrorResponse((
            Status::Unauthorized,
            "Geçersiz ehliyet".to_string(),
        )));
    }

    let claims = Claims {
        sub: usr.id,
        role: "user".to_string(),
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 10 * 60 * 60,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(settings.jwt_secret.as_bytes()),
    )
    .unwrap();

    Ok(SuccessResponse((
        Status::Ok,
        Json(SignInResponse { token }),
    )))
}

#[post("/sign-up", data = "<sign_up_request>")]
pub async fn sign_up(
    db: &State<DatabaseConnection>,
    sign_up_request: Json<SignUpRequest>,
) -> Response<String> {
    let db = db as &DatabaseConnection;
    if User::find()
        .filter(user::Column::Email.eq(&sign_up_request.email))
        .one(db)
        .await?
        .is_some()
    {
        return Err(ErrorResponse((
            Status::UnprocessableEntity,
            "Bu email adresiyle kayıtlı bir abone mevcut".to_string(),
        )));
    }

    User::insert(user::ActiveModel {
        email: Set(sign_up_request.email.to_owned()),
        password: Set(hash(&sign_up_request.password, DEFAULT_COST).unwrap()),
        first_name: Set(sign_up_request.firstname.to_owned()),
        sur_name: Set(sign_up_request.surname.to_owned()),
        ..Default::default()
    })
    .exec(db)
    .await?;

    Ok(SuccessResponse((
        Status::Created,
        "Kullanıcı hesabı oluşturuldu".to_string(),
    )))
}
