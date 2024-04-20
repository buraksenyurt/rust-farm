use crate::cache::update_cache_if_needed;
use crate::entity::{Note, NoteForm};
use crate::utility::get_file_path;
use handlebars::Handlebars;
use log::{error, info};
use rand::prelude::SliceRandom;
use std::fs;
use std::sync::Arc;
use warp::http::{Response, StatusCode};
use warp::reject::Reject;
use warp::{reject, Rejection, Reply};

#[derive(Debug)]
struct NoteError {
    pub message: String,
}
impl Reject for NoteError {}
pub struct Handler {}

impl Handler {
    pub async fn index_handler() -> Result<impl Reply, Rejection> {
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

    pub async fn note_form_handler() -> Result<impl Reply, Rejection> {
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

    pub async fn add_note_handler(note_data: NoteForm) -> Result<impl Reply, Rejection> {
        let path = get_file_path("notes.json");
        let notes_file_content = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
        let mut notes: Vec<Note> =
            serde_json::from_str(&notes_file_content).unwrap_or_else(|_| vec![]);
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
            Ok(_) => {
                info!("New note has been added.");
                Ok(warp::reply::with_status(
                    "Note successfully added!",
                    StatusCode::CREATED,
                ))
            }
            Err(e) => {
                error!("{}", e);
                Err(warp::reject::custom(NoteError {
                    message: "Failed to write to file".to_string(),
                }))
            }
        }
    }
}
