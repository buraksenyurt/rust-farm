mod api;
mod model;
mod repository;

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/")]
fn hello_world() -> Result<Json<String>, Status> {
    Ok(Json("Mongo DB için hello world".to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello_world])
}
