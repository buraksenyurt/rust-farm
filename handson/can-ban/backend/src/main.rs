mod api;
mod db;
mod handlers;
mod models;
mod state;
mod utility;

use crate::handlers::reports::ReportHandler;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use db::DbContext;
use handlers::work_items::WorkItemHandler;
use openssl::pkey::{PKey, Private};
use openssl::ssl::{SslAcceptor, SslMethod};
use state::AppState;
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key(&load_encrypted_private_key())
        .unwrap();
    builder
        .set_certificate_chain_file("../cert/cert.pem")
        .unwrap();

    let db_context = Mutex::new(DbContext::new(false).expect("Failed to init database"));
    let data = Data::new(AppState { db_context });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "PATCH"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/api/items", web::post().to(WorkItemHandler::create))
            .route("/api/items", web::get().to(WorkItemHandler::get_actives))
            .route("/api/items/all", web::get().to(WorkItemHandler::get_all))
            .route("/api/items/{id}", web::get().to(WorkItemHandler::get))
            .route("/api/items/{id}", web::delete().to(WorkItemHandler::delete))
            .route(
                "/api/items",
                web::patch().to(WorkItemHandler::move_to_archive),
            )
            .route("/api/items", web::put().to(WorkItemHandler::update_state))
            .route(
                "/api/items/stats/count",
                web::get().to(WorkItemHandler::get_count),
            )
            .route(
                "/api/items/report/summary",
                web::get().to(ReportHandler::get_board_summary_report),
            )
    })
    .bind_openssl("localhost:4448", builder)?
    .run()
    .await
}

fn load_encrypted_private_key() -> PKey<Private> {
    let mut file = File::open("../cert/key.pem").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    PKey::private_key_from_pem_passphrase(&buffer, b"P@ssw0rd").unwrap()
}
