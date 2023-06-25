use crate::controller::{ErrorResponse, Response, SuccessResponse};
use crate::entity::prelude::{Developer, Game};
use crate::entity::{developer, game};
use crate::messages::{
    CreateDeveloperRequest, DeveloperListResponse, DeveloperResponse, GameListResponse,
    GameResponse,
};
use crate::security::AuthenticatedUser;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryOrder};
use std::time::SystemTime;

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

#[put("/<id>", data = "<payload>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
    payload: Json<CreateDeveloperRequest>,
) -> Response<Json<DeveloperResponse>> {
    let db = db as &DatabaseConnection;
    match Developer::find_by_id(id).one(db).await? {
        Some(d) => {
            let mut dvlpr: developer::ActiveModel = d.into();
            dvlpr.level = Set(payload.level);
            dvlpr.fullname = Set(payload.fullname.to_owned());
            dvlpr.about = Set(payload.about.to_owned());
            dvlpr.modified_at = Set(Option::from(DateTimeUtc::from(SystemTime::now())));
            let updated = dvlpr.update(db).await?;
            Ok(SuccessResponse((
                Status::Ok,
                Json(DeveloperResponse {
                    id: updated.id,
                    fullname: updated.fullname,
                    about: updated.about,
                    level: updated.level,
                }),
            )))
        }
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "Programcı bilgisi bulunamadı.".to_string(),
            )))
        }
    }
}

#[delete("/<id>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<String> {
    let db = db as &DatabaseConnection;
    match Developer::find_by_id(id).one(db).await? {
        Some(d) => {
            d.delete(db).await?;
            Ok(SuccessResponse((
                Status::Ok,
                "Programcı bilgileri silindi".to_string(),
            )))
        }
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "Programcı bulunamadı".to_string(),
            )))
        }
    }
}

#[get("/<id>/games")]
pub async fn get_developer_games(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<GameListResponse>> {
    let db = db as &DatabaseConnection;
    match Developer::find_by_id(id).one(db).await? {
        Some(d) => {
            let games: Vec<game::Model> = d.find_related(Game).all(db).await?;
            Ok(SuccessResponse((
                Status::Ok,
                Json(GameListResponse {
                    total_count: games.len(),
                    games: games
                        .iter()
                        .map(|g| GameResponse {
                            id: g.id,
                            developer_id: g.developer_id,
                            title: g.title.to_owned(),
                            summary: g.summary.to_owned(),
                            year: g.year.to_owned(),
                        })
                        .collect(),
                }),
            )))
        }
        None => Err(ErrorResponse((
            Status::NotFound,
            "Programcı bulunamadı".to_string(),
        ))),
    }
}
