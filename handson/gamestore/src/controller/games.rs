use crate::controller::{ErrorResponse, Response, SuccessResponse};
use crate::entity::game;
use crate::entity::prelude::Game;
use crate::messages::{CreateGameRequest, GameListResponse, GameResponse};
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
) -> Response<Json<GameListResponse>> {
    let db = db as &DatabaseConnection;
    let games = Game::find()
        .order_by_desc(game::Column::Id)
        .all(db)
        .await?
        .iter()
        .map(|g| GameResponse {
            id: g.id,
            developer_id: g.developer_id,
            title: g.title.to_owned(),
            summary: g.summary.to_owned(),
            year: g.year.to_owned(),
        })
        .collect::<Vec<_>>();
    Ok(SuccessResponse((
        Status::Ok,
        Json(GameListResponse {
            total_count: games.len(),
            games,
        }),
    )))
}

#[post("/", data = "<payload>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    payload: Json<CreateGameRequest>,
) -> Response<Json<GameResponse>> {
    let game = game::ActiveModel {
        user_id: Set(user.user_id),
        developer_id: Set(payload.developer_id),
        title: Set(payload.title.to_owned()),
        summary: Set(payload.summary.to_owned()),
        year: Set(payload.year.to_owned()),
        ..Default::default()
    };
    let db = db as &DatabaseConnection;
    let created = game.insert(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(GameResponse::from(&created)),
    )))
}

#[get("/<id>")]
pub async fn get_detail(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<GameResponse>> {
    let db = db as &DatabaseConnection;
    let result = Game::find_by_id(id).one(db).await?;
    match result {
        Some(g) => Ok(SuccessResponse((
            Status::Ok,
            Json(GameResponse::from(&g)),
        ))),
        None => Err(ErrorResponse((
            Status::NotFound,
            "Oyun bulunamadı. ID bilgisini kontrol edin".to_string(),
        ))),
    }
}

#[put("/<id>", data = "<payload>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
    payload: Json<CreateGameRequest>,
) -> Response<Json<GameResponse>> {
    let db = db as &DatabaseConnection;
    match Game::find_by_id(id).one(db).await? {
        Some(g) => {
            let mut g: game::ActiveModel = g.into();
            g.developer_id = Set(payload.developer_id);
            g.title = Set(payload.title.to_owned());
            g.summary = Set(payload.summary.to_owned());
            g.year = Set(payload.year.to_owned());
            g.modified_at = Set(Option::from(DateTimeUtc::from(SystemTime::now())));
            let updated = g.update(db).await?;
            Ok(SuccessResponse((
                Status::Ok,
                Json(GameResponse::from(&updated)),
            )))
        }
        None => Err(ErrorResponse((
            Status::NotFound,
            "Programcı bilgisi bulunamadı.".to_string(),
        ))),
    }
}

#[delete("/<id>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<String> {
    let db = db as &DatabaseConnection;
    match Game::find_by_id(id).one(db).await? {
        Some(g) => {
            g.delete(db).await?;
            Ok(SuccessResponse((
                Status::Ok,
                "Oyun bilgileri silindi".to_string(),
            )))
        }
        None => Err(ErrorResponse((
            Status::NotFound,
            "Oyun bilgisi bulunamadı".to_string(),
        ))),
    }
}
