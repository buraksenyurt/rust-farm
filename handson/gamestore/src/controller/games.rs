use crate::controller::{Response, SuccessResponse};
use crate::entity::game;
use crate::entity::prelude::Game;
use crate::messages::{CreateGameRequest, GameListResponse, GameResponse};
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
        Json(GameResponse {
            id: created.id,
            developer_id: created.developer_id,
            title: created.title,
            summary: created.summary,
            year: created.year,
        }),
    )))
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
