mod app_state;
mod db_context;
mod handler;
mod model;
mod test;

use crate::app_state::AppState;
use crate::db_context::DbContext;
use crate::handler::Handler;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let db_context = Mutex::new(DbContext::new().expect("Failed to init database"));
    let data = Data::new(AppState { db_context });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .route("/api/items", web::post().to(Handler::create))
            .route("/api/items", web::get().to(Handler::get_all))
            .route("/api/items/{id}", web::get().to(Handler::get))
            .route("/api/items", web::put().to(Handler::update_state))
    })
    .bind("localhost:4448")?
    .run()
    .await
}
