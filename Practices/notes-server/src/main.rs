mod cache;
mod entity;
mod utility;

use crate::cache::update_cache_if_needed;
use crate::utility::get_file_path;
use handlebars::Handlebars;
use rand::seq::SliceRandom;
use std::sync::Arc;
use warp::{http::Response, reject, reply::Reply, Filter, Rejection};

async fn render() -> Result<impl Reply, Rejection> {
    let mut handlebars = Handlebars::new();
    if handlebars
        .register_template_file("index", get_file_path("templates/index.hbs"))
        .is_err()
    {
        return Err(reject::not_found());
    }
    let handlebars = Arc::new(handlebars);

    let cache = update_cache_if_needed().await;
    let cache = cache.lock().await;
    let cached_notes = cache.as_ref().unwrap();

    let note = cached_notes.notes.choose(&mut rand::thread_rng());
    let rendered = match handlebars.render("index", &note) {
        Ok(rendered) => rendered,
        Err(_) => return Err(reject::not_found()),
    };

    Ok(Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(rendered)
        .unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let route = warp::path!("note").and_then(render);

    println!("Server is running on localhost:5555");
    warp::serve(route).run(([0, 0, 0, 0], 5555)).await;

    Ok(())
}
