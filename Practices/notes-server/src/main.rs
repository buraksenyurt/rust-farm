use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use warp::{http::Response, Filter};

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    id: usize,
    title: String,
    body: String,
    publisher: String,
    author: String,
    year: usize,
    month: String,
    day: usize,
}

#[tokio::main]
async fn main() {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("index", "./templates/index.hbs")
        .expect("Template dosyası yüklenemedi");

    let handlebars = Arc::new(handlebars);

    let route = warp::path!("note").map(move || {
        let hb = handlebars.clone();

        let mut file = File::open("notes.json").expect("JSON dosyası okunamadı");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read JSON");

        let note: Note = from_str(&contents).expect("JSON serileştirmede problem var");

        let rendered = hb
            .render("index", &note)
            .unwrap();

        Response::builder()
            .header("Content-Type", "text/html; charset=utf-8")
            .body(rendered)
            .unwrap()
    });

    println!("Server is active on localhost:5555");
    warp::serve(route).run(([127, 0, 0, 1], 5555)).await;
}
