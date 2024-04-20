mod cache;
mod entity;
mod handler;
mod utility;

use crate::handler::Handler;
use log::info;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let index_route = warp::path!("note")
        .and(warp::get())
        .and_then(Handler::index_handler);

    let note_form_route = warp::path!("note" / "add")
        .and(warp::get())
        .and_then(Handler::note_form_handler);

    let add_note_route = warp::path!("note" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(Handler::add_note_handler);

    let routes = index_route.or(note_form_route).or(add_note_route);

    info!("Server is running on localhost:5555");
    warp::serve(routes).run(([0, 0, 0, 0], 5555)).await;

    Ok(())
}
