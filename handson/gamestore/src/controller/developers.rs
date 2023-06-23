use crate::controller::{Response, SuccessResponse};
use crate::entity::developer;
use crate::entity::prelude::Developer;
use crate::messages::{DeveloperListResponse, DeveloperResponse};
use crate::security::AuthenticatedUser;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};

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

#[post("/")]
pub async fn create() -> Response<String> {
    todo!()
}

#[get("/<id>")]
pub async fn get_detail(id: u32) -> Response<String> {
    todo!()
}

#[put("/<id>")]
pub async fn update(id: u32) -> Response<String> {
    todo!()
}

#[delete("/<id>")]
pub async fn delete(id: u32) -> Response<String> {
    todo!()
}
