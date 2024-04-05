use crate::controller::*;
use actix_web::{web, App, HttpServer};

mod controller;
mod db;
mod model;
mod test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/challenges", web::post().to(create))
            .route("/challenges", web::get().to(get_all))
            .route("/challenges/{id}", web::get().to(get_by_id))
            .route("/challenges/{id}", web::delete().to(delete))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
