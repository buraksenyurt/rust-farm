mod app_settings;
mod data;
mod migrator;

#[macro_use]
extern crate rocket;

use crate::app_settings::AppSettings;
use crate::migrator::Migrator;
use sea_orm_migration::MigratorTrait;

#[get("/")]
fn index() -> &'static str {
    "shall we begin!?"
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

    rocket::build().mount("/", routes![index])
}
