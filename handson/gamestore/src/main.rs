mod app_settings;
mod controller;
mod data;
mod entity;
mod fairings;
mod messages;
mod migrator;

#[macro_use]
extern crate rocket;

use crate::app_settings::AppSettings;
use crate::controller::*;
use crate::fairings::*;
use crate::migrator::Migrator;
use rocket::http::Status;
use sea_orm_migration::MigratorTrait;

#[get("/")]
fn index() -> Response<String> {
    Ok(SuccessResponse((
        Status::Ok,
        "shall we begin!?".to_string(),
    )))
}

#[launch]
async fn rocket() -> _ {
    let app_sets = AppSettings::default();
    let db = match data::connect(&app_sets).await {
        Ok(db) => db,
        Err(e) => panic!("{}", e),
    };

    match Migrator::up(&db, None).await {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }

    rocket::build()
        .attach(CORS)
        .manage(db)
        .mount("/", routes![options])
        .mount("/", routes![index])
        .mount(
            "/auth",
            routes![controller::auth::sign_in, controller::auth::sign_up,],
        )
        .mount(
            "/games",
            routes![
                controller::games::index,
                controller::games::create,
                controller::games::get_detail,
                controller::games::update,
                controller::games::delete,
            ],
        )
        .mount(
            "/developers",
            routes![
                controller::developers::index,
                controller::developers::create,
                controller::developers::get_detail,
                controller::developers::update,
                controller::developers::delete,
            ],
        )
}
