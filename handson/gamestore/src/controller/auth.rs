use crate::controller::{ErrorResponse, Response, SuccessResponse};
use crate::entity::prelude::User;
use crate::entity::user;
use crate::messages::{SignInRequest, SignInResponse, SignUpRequest};
use bcrypt::{hash, DEFAULT_COST};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::*;

#[post("/sign-in", data = "<sign_in_request>")]
pub async fn sign_in(
    db: &State<DatabaseConnection>,
    sign_in_request: Json<SignInRequest>,
) -> Response<SignInResponse> {
    let db = db as &DatabaseConnection;
    todo!()
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
        password: Set(hash(sign_up_request.password.to_owned(), DEFAULT_COST).unwrap()),
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
