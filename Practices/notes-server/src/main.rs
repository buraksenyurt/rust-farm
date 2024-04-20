mod cache;
mod entity;
mod utility;

use crate::cache::update_cache_if_needed;
use crate::entity::{Note, NoteForm};
use crate::utility::get_file_path;
use handlebars::Handlebars;
use rand::seq::SliceRandom;
use std::fs;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::{http::Response, reject, reply::Reply, Filter, Rejection};

async fn index_handler() -> Result<impl Reply, Rejection> {
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

#[derive(Debug)]
struct NoteError {
    pub message: String,
}
impl Reject for NoteError {}

async fn add_note_handler(note_data: NoteForm) -> Result<impl Reply, Rejection> {
    let path = get_file_path("notes.json");
    let notes_file_content = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    let mut notes: Vec<Note> = serde_json::from_str(&notes_file_content).unwrap_or_else(|_| vec![]);
    let new_note = Note {
        id: notes.len() + 1,
        title: note_data.title,
        body: note_data.body,
        publisher: note_data.publisher,
        author: note_data.author,
        media_type: note_data.media_type,
        year: note_data.year,
        month: note_data.month,
        day: note_data.day,
        externals: Option::from(note_data.externals),
    };

    notes.push(new_note);

    let path = get_file_path("notes.json");
    match fs::write(path, serde_json::to_string(&notes).unwrap()) {
        Ok(_) => Ok(warp::reply::with_status(
            "Note successfully added!",
            StatusCode::CREATED,
        )),
        Err(_) => Err(warp::reject::custom(NoteError {
            message: "Failed to write to file".to_string(),
        })),
    }
}

async fn note_form_handler() -> Result<impl Reply, Rejection> {
    let mut handlebars = Handlebars::new();
    if handlebars
        .register_template_file("noteForm", get_file_path("templates/noteForm.hbs"))
        .is_err()
    {
        return Err(reject::not_found());
    }
    let handlebars = Arc::new(handlebars);

    let rendered = handlebars
        .render("noteForm", &{})
        .map_err(|_| reject::not_found())?;

    Ok(Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(rendered)
        .unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["GET", "POST"]);

    let index_route = warp::path!("note").and(warp::get()).and_then(index_handler);

    let note_form_route = warp::path!("note" / "add")
        .and(warp::get())
        .and_then(note_form_handler);

    let add_note_route = warp::path!("note" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add_note_handler);

    let routes = index_route.or(note_form_route).or(add_note_route);

    println!("Server is running on localhost:5555");
    warp::serve(routes).run(([0, 0, 0, 0], 5555)).await;

    Ok(())
}
