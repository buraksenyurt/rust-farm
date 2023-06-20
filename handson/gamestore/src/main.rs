mod migrator;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "shall we begin!?"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
