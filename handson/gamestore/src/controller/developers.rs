use crate::controller::{ErrorResponse, Response, SuccessResponse};
use crate::entity::developer;
use crate::entity::prelude::Developer;
use crate::messages::{CreateDeveloperRequest, DeveloperListResponse, DeveloperResponse};
use crate::security::AuthenticatedUser;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder};

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<DeveloperListResponse>> {
    let db = db as &DatabaseConnection;
    let developers = Developer::find()
        .order_by_desc(developer::Column::Id)
        .all(db)
        .await?
        .iter()
        .map(|d| DeveloperResponse {
            id: d.id,
            fullname: d.fullname.to_owned(),
            about: d.about.to_owned(),
            level: d.level,
        })
        .collect::<Vec<_>>();
    Ok(SuccessResponse((
        Status::Ok,
        Json(DeveloperListResponse {
            total_count: developers.len(),
            developers,
        }),
    )))
}

#[post("/", data = "<payload>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    payload: Json<CreateDeveloperRequest>,
) -> Response<Json<DeveloperResponse>> {
    let developer = developer::ActiveModel {
        user_id: Set(user.user_id),
        fullname: Set(payload.fullname.to_owned()),
        level: Set(payload.level),
        about: Set(payload.about.to_owned()),
        ..Default::default()
    };
    let db = db as &DatabaseConnection;
    let created = developer.insert(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(DeveloperResponse {
            id: created.id,
            fullname: created.fullname,
            level: created.level,
            about: created.about,
        }),
    )))
}

#[get("/<id>")]
pub async fn get_detail(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<DeveloperResponse>> {
    let db = db as &DatabaseConnection;
    let result = Developer::find_by_id(id).one(db).await?;
    match result {
        Some(d) => Ok(SuccessResponse((
            Status::Ok,
            Json(DeveloperResponse {
                id: d.id,
                fullname: d.fullname,
                about: d.about,
                level: d.level,
            }),
        ))),
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "Programcı bilgisi bulunamadı. ID bilgisini kontrol edin".to_string(),
            )))
        }
    }
}

#[put("/<id>")]
pub async fn update(id: u32) -> Response<String> {
    todo!()
}

#[delete("/<id>")]
pub async fn delete(id: u32) -> Response<String> {
    todo!()
}
