use handlebars::Handlebars;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use warp::{http::Response, reject, reply::Reply, Filter, Rejection};

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    id: usize,
    title: String,
    body: String,
    publisher: String,
    author: String,
    #[serde(rename = "mediaType")]
    media_type: MediaType,
    year: usize,
    month: String,
    day: usize,
    externals: Vec<External>,
}

#[derive(Serialize, Deserialize, Debug)]
struct External {
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
enum MediaType{
    Gazete,
    Dergi,
    Dijital,
    Kitap,
    Podcast,
    Medium,
    Unknown
}

async fn render() -> Result<impl Reply, Rejection> {
    let mut handlebars = Handlebars::new();
    if handlebars
        .register_template_file("index", "./templates/index.hbs")
        .is_err()
    {
        return Err(reject::not_found());
    }

    let handlebars = Arc::new(handlebars);

    let mut file = match File::open("notes.json") {
        Ok(file) => file,
        Err(_) => return Err(reject::not_found()),
    };
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return Err(reject::not_found());
    }

    let notes: Vec<Note> = match from_str(&contents) {
        Ok(notes) => notes,
        Err(e) => {
            println!("{}",e);
            return Err(reject::not_found());
        },
    };

    let note = notes.choose(&mut rand::thread_rng());
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
    warp::serve(route).run(([127, 0, 0, 1], 5555)).await;

    Ok(())
}
