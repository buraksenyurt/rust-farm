mod model;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::http::header;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::{info, warn};
use model::WorkItem;
use std::collections::HashMap;
use std::sync::Mutex;

async fn create(item: web::Json<WorkItem>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let id = items.len() as u32 + 1;
    let mut new_item = item.into_inner();
    new_item.id = id;
    items.insert(id, new_item);
    info!("New item has been added");
    HttpResponse::Created().json("Item created")
}

async fn get(id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    if let Some(item) = items.get(&id.into_inner()) {
        HttpResponse::Ok().json(item)
    } else {
        warn!("Failed to find");
        HttpResponse::NotFound().finish()
    }
}

struct AppState {
    items: Mutex<HashMap<u32, WorkItem>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let data = Data::new(AppState {
        items: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .route("/api/items", web::post().to(create))
            .route("/api/items/{id}", web::get().to(get))
    })
    .bind("localhost:4448")?
    .run()
    .await
}
