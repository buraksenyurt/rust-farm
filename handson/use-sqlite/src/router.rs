use crate::database::{
    create_game, delete_game, load_categories, load_games, open_connection, update_game,
};
use crate::model::{NewGamePayload, UpdateGamePayload, UpsertGame};
use actix_web::{web, Responder, Result};

pub async fn get_categories() -> Result<impl Responder> {
    let conn = &mut open_connection();
    let category_list = load_categories(conn);
    Ok(web::Json(category_list.to_vec()))
}

pub async fn post_game(game: web::Json<NewGamePayload>) -> Result<impl Responder> {
    let conn = &mut open_connection();
    let upsert_game = UpsertGame::new(game.category_id, game.title.as_str(), game.stars);
    let insert_result = create_game(conn, upsert_game);
    Ok(web::Json(insert_result))
}

pub async fn put_game(game: web::Json<UpdateGamePayload>) -> Result<impl Responder> {
    let conn = &mut open_connection();
    let upsert_game = UpsertGame::new(game.category_id, game.title.as_str(), game.stars);
    let update_result = update_game(conn, game.id, upsert_game);
    Ok(web::Json(update_result))
}

pub async fn del_game(game_name: web::Path<String>) -> Result<impl Responder> {
    let conn = &mut open_connection();
    let delete_result = delete_game(conn, game_name.as_str());
    Ok(web::Json(delete_result))
}

pub async fn get_games() -> Result<impl Responder> {
    let conn = &mut open_connection();
    let game_list = load_games(conn);
    Ok(web::Json(game_list.to_vec()))
}
