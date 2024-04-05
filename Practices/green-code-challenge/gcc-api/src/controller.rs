use crate::db::{create_challenge, delete_challenge, select_challenge_by_id, select_challenges};
use crate::model::Challenge;
use actix_web::{web, HttpResponse};
use rusqlite::Connection;

pub async fn create(item: web::Json<Challenge>) -> HttpResponse {
    let conn = Connection::open("green.db").expect("DB Connection error!");
    match create_challenge(&conn, item.into_inner()) {
        Ok(_) => HttpResponse::Created().json("Challenge has been created"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Fatal Error: {:?}", e)),
    }
}

pub async fn get_all() -> HttpResponse {
    let conn = Connection::open("green.db").expect("DB Connection error!");
    match select_challenges(&conn) {
        Ok(challenges) => HttpResponse::Ok().json(challenges),
        Err(e) => HttpResponse::InternalServerError().body(format!("Fatal Error: {:?}", e)),
    }
}

pub async fn get_by_id(path: web::Path<(i32,)>) -> HttpResponse {
    let conn = Connection::open("green.db").expect("DB Connection error!");
    let id = path.into_inner().0;
    match select_challenge_by_id(&conn, id) {
        Ok(challenge) => HttpResponse::Ok().json(challenge),
        Err(e) => HttpResponse::InternalServerError().body(format!("Fatal Error: {:?}", e)),
    }
}

pub async fn delete(path: web::Path<(i32,)>) -> HttpResponse {
    let conn = Connection::open("green.db").expect("DB Connection error!");
    let challenge_id = path.into_inner().0;

    match delete_challenge(&conn, challenge_id) {
        Ok(_) => {
            HttpResponse::Ok().json(format!("Challenge Id: {} has been deleted.", challenge_id))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Fatal Error: {:?}", e)),
    }
}
