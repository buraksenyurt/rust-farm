mod cache;
mod entity;
mod handler;
mod hb_engine;
mod utility;

use crate::entity::NoteForm;
use crate::handler::Handler;
use crate::hb_engine::create_handlebars;
use handlebars::Handlebars;
use log::info;
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let handlebars = create_handlebars().await;

    let index_route = warp::path!("note")
        .and(warp::get())
        .and(with_handlebars(handlebars.clone()))
        .and_then(Handler::index_handler);

    let list_notes_route = warp::path!("note" / "list")
        .and(warp::get())
        .and(with_handlebars(handlebars.clone()))
        .and_then(Handler::get_all_handler);

    let list_notes_ordered_route = warp::path!("note" / "ordered" / String / String)
        .and(warp::get())
        .and(with_handlebars(handlebars.clone()))
        .and_then(|column, order, handlebars: Arc<Handlebars<'static>>| {
            Handler::get_all_with_order_handler(column, order, handlebars)
        });

    let note_form_route = warp::path!("note" / "add")
        .and(warp::get())
        .and(with_handlebars(handlebars.clone()))
        .and_then(Handler::note_form_handler);

    let detail_note_route = warp::path!("note" / "detail" / usize)
        .and(warp::get())
        .and(with_handlebars(handlebars.clone()))
        .and_then(|id, handlebars: Arc<Handlebars<'static>>| Handler::get_by_id(id, handlebars));

    let add_note_route = warp::path!("note" / "create")
        .and(warp::post())
        .and(warp::body::json::<NoteForm>())
        .and_then(Handler::add_note_handler);

    let routes = index_route
        .or(note_form_route)
        .or(add_note_route)
        .or(list_notes_route)
        .or(detail_note_route)
        .or(list_notes_ordered_route);

    info!("Server is running on localhost:5555");
    warp::serve(routes).run(([0, 0, 0, 0], 5555)).await;

    Ok(())
}

fn with_handlebars(
    handlebars: Arc<Handlebars>,
) -> impl Filter<Extract = (Arc<Handlebars>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || handlebars.clone())
}
