use crate::controller::*;
use crate::db::init_db;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use apistos::app::OpenApiWrapper;
use apistos::info::Info;
use apistos::server::Server;
use apistos::spec::Spec;
use apistos::web::{delete, get, post, resource, scope};
use log::info;
use rusqlite::Connection;
use std::env;
use std::error::Error;
use std::net::Ipv4Addr;
use std::path::Path;

mod controller;
mod db;
mod model;
mod test;

pub const DB_NAME: &str = "green.db";

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db_path = Path::new(DB_NAME);
    let conn = Connection::open(db_path).expect("Open Connection Error");
    init_db(&conn).expect("DB Create Error");

    let server = HttpServer::new(move || {
        let api_spec = Spec {
            info: Info {
                title: "Green Code Challenges API".to_string(),
                description: Some("Green Code Challenges API !".to_string()),
                ..Default::default()
            },
            servers: vec![Server {
                url: "/api/v3".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        App::new()
            .document(api_spec)
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST","DELETE"])
                    .allow_any_header(),
            )
            .service(
                scope("/api").service(
                    scope("/challenge")
                        .service(resource("").route(get().to(get_all)))
                        .service(resource("/{id}").route(get().to(get_by_id)))
                        .service(resource("").route(post().to(create)))
                        .service(resource("/{id}").route(delete().to(remove))),
                ),
            )
            .build("/openapi.json")
    });
    info!("Server is online at {}:44444", Ipv4Addr::UNSPECIFIED);
    server.bind((Ipv4Addr::UNSPECIFIED, 44444))?.run().await
}
