use crate::controller::{Response, SuccessResponse};
use crate::entity::game;
use crate::entity::prelude::Game;
use crate::messages::{GameListResponse, GameResponse};
use crate::security::AuthenticatedUser;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};

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
