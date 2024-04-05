use crate::db::{create_challenge, delete_challenge, select_challenge_by_id, select_challenges};
use crate::model::Challenge;
use crate::DB_NAME;
use actix_web::{web, HttpResponse};
use apistos::{api_operation, ApiComponent};
use log::{info, warn};
use rusqlite::Connection;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, ApiComponent, JsonSchema)]
pub struct MessageResponse {
    message: String,
}

#[derive(Serialize, ApiComponent, JsonSchema)]
pub struct ErrorResponse {
    error: String,
}

#[api_operation(summary = "Create an new Challenge")]
pub async fn create(item: web::Json<Challenge>) -> HttpResponse {
    warn!("Create Request");
    let conn = Connection::open(DB_NAME).expect("DB Connection error!");
    match create_challenge(&conn, item.into_inner()) {
        Ok(_) => HttpResponse::Created().json(MessageResponse {
            message: "Challenge has been created".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal Error: {:?}", e),
        }),
    }
}

#[api_operation(summary = "Get all registered challenges")]
pub async fn get_all() -> HttpResponse {
    let conn = Connection::open(DB_NAME).expect("DB Connection error!");
    match select_challenges(&conn) {
        Ok(challenges) => HttpResponse::Ok().json(challenges),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal Error: {:?}", e),
        }),
    }
}

#[api_operation(summary = "Get challenge by identity")]
pub async fn get_by_id(path: web::Path<(i32,)>) -> HttpResponse {
    let conn = Connection::open(DB_NAME).expect("DB Connection error!");
    let id = path.into_inner().0;
    match select_challenge_by_id(&conn, id) {
        Ok(Some(challenge)) => HttpResponse::Ok().json(challenge),
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "Challenge not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal Error: {:?}", e),
        }),
    }
}

#[api_operation(summary = "Delete challenge with id")]
pub async fn remove(path: web::Path<(i32,)>) -> HttpResponse {
    let conn = Connection::open(DB_NAME).expect("DB Connection error!");
    let challenge_id = path.into_inner().0;
    match delete_challenge(&conn, challenge_id) {
        Ok(_) => HttpResponse::Ok().json(MessageResponse {
            message: format!("Challenge Id: {} has been deleted.", challenge_id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal Error: {:?}", e),
        }),
    }
}
