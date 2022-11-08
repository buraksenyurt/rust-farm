mod api;
mod model;
mod repository;

#[macro_use]
extern crate rocket;

use crate::api::product_api::create_product;
use crate::repository::db::Db;
use rocket::http::Status;
use rocket::serde::json::Json;

// #[get("/")]
// fn hello_world() -> Result<Json<String>, Status> {
//     Ok(Json("Mongo DB için hello world".to_string()))
// }

#[launch]
fn rocket() -> _ {
    let mongo_db = Db::init();
    rocket::build()
        .manage(mongo_db)
        .mount("/", routes![create_product])
}
